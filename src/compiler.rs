use crate::ast::*;
use crate::bytecode::{BytecodeModule, ClassEntry, ConstantPool, FuncEntry, PoolEntry};
use crate::instruction::Instruction;
use crate::value::Value;

pub struct Compiler {
    constants: ConstantPool,
    code: Vec<Instruction>,
    func_entries: Vec<FuncEntry>,
    class_entries: Vec<ClassEntry>,
    class_fields: Vec<(String, Vec<(String, Value)>)>,
    interface_table: Vec<(String, Vec<InterfaceMethod>)>,
    loops: Vec<Vec<usize>>,
    source_dir: Option<String>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            constants: ConstantPool::new(),
            code: Vec::new(),
            func_entries: Vec::new(),
            class_entries: Vec::new(),
            class_fields: Vec::new(),
            interface_table: Vec::new(),
            loops: Vec::new(),
            source_dir: None,
        }
    }

    pub fn with_source_dir(dir: String) -> Self {
        Compiler {
            constants: ConstantPool::new(),
            code: Vec::new(),
            func_entries: Vec::new(),
            class_entries: Vec::new(),
            class_fields: Vec::new(),
            interface_table: Vec::new(),
            loops: Vec::new(),
            source_dir: Some(dir),
        }
    }

    fn s(&mut self, s: &str) -> u32 { self.constants.add_string(s) }
    fn n(&mut self, n: f64) -> u32 { self.constants.add_number(n) }
    fn b(&mut self, b: bool) -> u32 { self.constants.add_bool(b) }
    fn null_idx(&mut self) -> u32 { self.constants.add_null() }

    fn emit(&mut self, inst: Instruction) { self.code.push(inst); }

    fn emit_at(&mut self, index: usize, inst: Instruction) { self.code[index] = inst; }

    fn current_pos(&self) -> usize { self.code.len() }

    fn resolve_module_path(&self, module: &str) -> String {
        let candidates = vec![
            module.to_string(),
            format!("{}.ls", module),
        ];
        let prefixed: Vec<String> = if let Some(ref dir) = self.source_dir {
            candidates.iter()
                .map(|p| format!("{}/{}", dir, p))
                .chain(candidates.clone())
                .collect()
        } else {
            candidates
        };
        for path in &prefixed {
            if std::path::Path::new(path).exists() { return path.clone(); }
        }
        format!("{}.ls", module)
    }

    fn absorb_func_compiler(&mut self, sub: Compiler, name: &str, params: &[String], captures: &[String]) -> FuncEntry {
        let const_mapping: Vec<u32> = sub.constants.items.iter().map(|entry| {
            match entry {
                PoolEntry::String(s) => self.constants.add_string(s),
                PoolEntry::Number(n) => self.constants.add_number(*n),
                PoolEntry::Bool(b) => self.constants.add_bool(*b),
                PoolEntry::Null => self.constants.add_null(),
            }
        }).collect();

        let name_idx = self.constants.add_string(name);
        let param_idxs: Vec<u32> = params.iter().map(|p| self.constants.add_string(p)).collect();
        let capture_idxs: Vec<u32> = captures.iter().map(|c| self.constants.add_string(c)).collect();

        let func_offset = self.func_entries.len() as u32;
        let class_offset = self.class_entries.len() as u32;

        for fe in &sub.func_entries {
            let rc = Self::remap_instructions(&fe.code, &const_mapping, func_offset, class_offset);
            self.func_entries.push(FuncEntry {
                name_idx: const_mapping[fe.name_idx as usize],
                params: fe.params.iter().map(|&i| const_mapping[i as usize]).collect(),
                captures: fe.captures.iter().map(|&i| const_mapping[i as usize]).collect(),
                code: rc,
            });
        }

        for ce in &sub.class_entries {
            let mut new_methods = Vec::new();
            for m in &ce.methods {
                let rc = Self::remap_instructions(&m.code, &const_mapping, func_offset, class_offset);
                new_methods.push(FuncEntry {
                    name_idx: const_mapping[m.name_idx as usize],
                    params: m.params.iter().map(|&i| const_mapping[i as usize]).collect(),
                    captures: m.captures.iter().map(|&i| const_mapping[i as usize]).collect(),
                    code: rc,
                });
            }
            self.class_entries.push(ClassEntry {
                name_idx: const_mapping[ce.name_idx as usize],
                field_names: ce.field_names.iter().map(|&i| const_mapping[i as usize]).collect(),
                methods: new_methods,
                publics: ce.publics.iter().map(|&i| const_mapping[i as usize]).collect(),
                constructor_params: ce.constructor_params.iter().map(|&i| const_mapping[i as usize]).collect(),
            });
        }

        let remapped_code = Self::remap_instructions(&sub.code, &const_mapping, func_offset, class_offset);
        FuncEntry { name_idx, params: param_idxs, captures: capture_idxs, code: remapped_code }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => { let idx = self.n(*n); self.emit(Instruction::Const(idx)); }
            Expr::String(s) => { let idx = self.s(s); self.emit(Instruction::Const(idx)); }
            Expr::Bool(b) => { let idx = self.b(*b); self.emit(Instruction::Const(idx)); }
            Expr::Null => { let idx = self.null_idx(); self.emit(Instruction::Const(idx)); }
            Expr::Identifier(name) => { let idx = self.s(name); self.emit(Instruction::LoadVar(idx)); }
            Expr::BinaryOp { op, left, right } => {
                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod | BinOp::Pow => {
                        self.compile_expr(left);
                        self.compile_expr(right);
                        let inst = match op {
                            BinOp::Add => Instruction::Add,
                            BinOp::Sub => Instruction::Sub,
                            BinOp::Mul => Instruction::Mul,
                            BinOp::Div => Instruction::Div,
                            BinOp::Mod => Instruction::Mod,
                            BinOp::Pow => Instruction::Pow,
                            _ => unreachable!(),
                        };
                        self.emit(inst);
                    }
                    BinOp::And => {
                        self.compile_expr(left);
                        self.emit(Instruction::Dup);
                        let jump = self.current_pos();
                        self.emit(Instruction::JumpIfFalse(0));
                        self.emit(Instruction::Pop);
                        self.compile_expr(right);
                        let end = self.current_pos();
                        self.emit_at(jump, Instruction::JumpIfFalse((end - jump) as i32));
                    }
                    BinOp::Or => {
                        self.compile_expr(left);
                        self.emit(Instruction::Dup);
                        let jump = self.current_pos();
                        self.emit(Instruction::JumpIfTrue(0));
                        self.emit(Instruction::Pop);
                        self.compile_expr(right);
                        let end = self.current_pos();
                        self.emit_at(jump, Instruction::JumpIfTrue((end - jump) as i32));
                    }
                    BinOp::Eq => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Eq); }
                    BinOp::Ne => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Ne); }
                    BinOp::Gt => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Gt); }
                    BinOp::Lt => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Lt); }
                    BinOp::Ge => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Ge); }
                    BinOp::Le => { self.compile_expr(left); self.compile_expr(right); self.emit(Instruction::Le); }
                }
            }
            Expr::UnaryOp { op: UnaryOp::Neg, expr } => { self.compile_expr(expr); self.emit(Instruction::Neg); }
            Expr::UnaryOp { op: UnaryOp::Not, expr } => { self.compile_expr(expr); self.emit(Instruction::Not); }
            Expr::Call { callee, args } => {
                for arg in args { self.compile_expr(arg); }
                if let Expr::Identifier(name) = callee.as_ref() {
                    if let Some(rest) = name.strip_prefix("__instantiate__") {
                        let idx = self.s(rest);
                        self.emit(Instruction::Instantiate(idx));
                        return;
                    }
                }
                self.compile_expr(callee);
                self.emit(Instruction::Call(args.len() as u32));
            }
            Expr::MethodCall { object, method, args } => {
                self.compile_expr(object);
                for arg in args { self.compile_expr(arg); }
                let m_idx = self.s(method);
                self.emit(Instruction::LoadMethod(m_idx));
                self.emit(Instruction::Call((args.len() + 1) as u32));
            }
            Expr::ListLiteral(items) => {
                for item in items { self.compile_expr(item); }
                self.emit(Instruction::MakeList(items.len() as u32));
            }
            Expr::MapLiteral(pairs) => {
                for (k, v) in pairs {
                    let k_idx = self.s(k);
                    self.emit(Instruction::Const(k_idx));
                    self.compile_expr(v);
                }
                self.emit(Instruction::MakeMap(pairs.len() as u32));
            }
            Expr::TypeOf(expr) => { self.compile_expr(expr); self.emit(Instruction::TypeOf); }
            Expr::Capitalize(expr) => { self.compile_expr(expr); self.emit(Instruction::Capitalize); }
            Expr::SuperCall { method, args } => {
                let self_idx = self.s("self");
                let sm_idx = self.s(&format!("__super_{}", method));
                self.emit(Instruction::LoadVar(self_idx));
                for arg in args { self.compile_expr(arg); }
                self.emit(Instruction::LoadMethod(sm_idx));
                self.emit(Instruction::Call((args.len() + 1) as u32));
            }
            Expr::Input => { self.emit(Instruction::Input); }
            Expr::Index { object, index } => {
                self.compile_expr(object);
                self.compile_expr(index);
                self.emit(Instruction::Call(1));
            }
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::StartHere(body) => { for s in body { self.compile_stmt(s); } }
            Stmt::VarDef { name, value } | Stmt::VarDecl { name, value } => {
                self.compile_expr(value);
                let idx = self.s(name);
                self.emit(Instruction::StoreVar(idx));
            }
            Stmt::Assign { name, value } => {
                self.compile_expr(value);
                let idx = self.s(name);
                self.emit(Instruction::StoreVar(idx));
            }
            Stmt::Say(expr) => { self.compile_expr(expr); self.emit(Instruction::Say); }
            Stmt::Ask { prompt, var } => {
                self.compile_expr(prompt);
                let idx = self.s(var);
                self.emit(Instruction::Ask(idx));
            }
            Stmt::ReadFile { filename, var } => {
                self.compile_expr(filename);
                let idx = self.s(var);
                self.emit(Instruction::ReadFile(idx));
            }
            Stmt::WriteFile { content, filename } => {
                self.compile_expr(content);
                self.compile_expr(filename);
                self.emit(Instruction::WriteFile);
            }
            Stmt::If { condition, body, otherwise } => {
                self.compile_expr(condition);
                let else_jump = self.current_pos();
                self.emit(Instruction::JumpIfFalse(0));
                for s in body { self.compile_stmt(s); }
                let end_jump = self.current_pos();
                self.emit(Instruction::Jump(0));
                let else_start = self.current_pos();
                self.emit_at(else_jump, Instruction::JumpIfFalse((else_start - else_jump) as i32));
                for s in otherwise { self.compile_stmt(s); }
                let end = self.current_pos();
                self.emit_at(end_jump, Instruction::Jump((end - end_jump) as i32));
            }
            Stmt::Repeat { times, body } => {
                let loop_id = self.loops.len();
                self.loops.push(Vec::new());
                self.compile_expr(times);
                let limit_var = format!("__repeat_limit_{}", loop_id);
                let counter_var = format!("__repeat_counter_{}", loop_id);
                let lv = self.s(&limit_var);
                let cv = self.s(&counter_var);
                let z = self.n(0.0);
                self.emit(Instruction::StoreVar(lv));
                self.emit(Instruction::Const(z));
                self.emit(Instruction::StoreVar(cv));
                let loop_start = self.current_pos();
                let lv2 = self.s(&limit_var);
                let cv2 = self.s(&counter_var);
                self.emit(Instruction::LoadVar(cv2));
                self.emit(Instruction::LoadVar(lv2));
                self.emit(Instruction::Lt);
                let exit_jump = self.current_pos();
                self.emit(Instruction::JumpIfFalse(0));
                for s in body { self.compile_stmt(s); }
                let cv3 = self.s(&counter_var);
                let o2 = self.n(1.0);
                self.emit(Instruction::LoadVar(cv3));
                self.emit(Instruction::Const(o2));
                self.emit(Instruction::Add);
                let cv4 = self.s(&counter_var);
                self.emit(Instruction::StoreVar(cv4));
                let after_jump = self.current_pos();
                self.emit(Instruction::Jump(-((after_jump - loop_start) as i32)));
                let exit = self.current_pos();
                self.emit_at(exit_jump, Instruction::JumpIfFalse((exit - exit_jump) as i32));
                self.loops.pop();
            }
            Stmt::ForEach { var, collection, body } => {
                let loop_id = self.loops.len();
                self.compile_expr(collection);
                self.emit(Instruction::MapToList);
                self.emit(Instruction::Dup);
                self.emit(Instruction::ListLen);
                let fl = self.s(&format!("__foreach_len_{}", loop_id));
                let fs = self.s(&format!("__foreach_list_{}", loop_id));
                let z = self.n(0.0);
                self.emit(Instruction::StoreVar(fl));
                self.emit(Instruction::StoreVar(fs));
                self.emit(Instruction::Const(z));
                let fi = self.s(&format!("__foreach_idx_{}", loop_id));
                self.emit(Instruction::StoreVar(fi));

                let loop_start = self.current_pos();
                let fi2 = self.s(&format!("__foreach_idx_{}", loop_id));
                let fl2 = self.s(&format!("__foreach_len_{}", loop_id));
                self.emit(Instruction::LoadVar(fi2));
                self.emit(Instruction::LoadVar(fl2));
                self.emit(Instruction::Lt);
                let exit_jump = self.current_pos();
                self.emit(Instruction::JumpIfFalse(0));

                let fi3 = self.s(&format!("__foreach_idx_{}", loop_id));
                let fs2 = self.s(&format!("__foreach_list_{}", loop_id));
                self.emit(Instruction::LoadVar(fi3));
                self.emit(Instruction::LoadVar(fs2));
                self.emit(Instruction::IndexGet);
                let v = self.s(var);
                self.emit(Instruction::StoreVar(v));

                for s in body { self.compile_stmt(s); }

                let fi4 = self.s(&format!("__foreach_idx_{}", loop_id));
                let o = self.n(1.0);
                self.emit(Instruction::LoadVar(fi4));
                self.emit(Instruction::Const(o));
                self.emit(Instruction::Add);
                let fi5 = self.s(&format!("__foreach_idx_{}", loop_id));
                self.emit(Instruction::StoreVar(fi5));
                self.emit(Instruction::Jump(-((self.current_pos() - loop_start) as i32)));
                let exit = self.current_pos();
                self.emit_at(exit_jump, Instruction::JumpIfFalse((exit - exit_jump) as i32));
            }
            Stmt::While { condition, body } => {
                let loop_start = self.current_pos();
                self.compile_expr(condition);
                let exit_jump = self.current_pos();
                self.emit(Instruction::JumpIfFalse(0));
                for s in body { self.compile_stmt(s); }
                self.emit(Instruction::Jump(-((self.current_pos() - loop_start) as i32)));
                let exit = self.current_pos();
                self.emit_at(exit_jump, Instruction::JumpIfFalse((exit - exit_jump) as i32));
            }
            Stmt::Block(stmts) => { for s in stmts { self.compile_stmt(s); } }
            Stmt::Return(Some(expr)) => { self.compile_expr(expr); self.emit(Instruction::Return); }
            Stmt::Return(None) => { let idx = self.null_idx(); self.emit(Instruction::Const(idx)); self.emit(Instruction::Return); }
            Stmt::Stop => { self.emit(Instruction::Stop); }
            Stmt::Exit(Some(expr)) => { self.compile_expr(expr); self.emit(Instruction::Exit); }
            Stmt::Exit(None) => { let idx = self.n(0.0); self.emit(Instruction::Const(idx)); self.emit(Instruction::Exit); }
            Stmt::Raise(expr) => { self.compile_expr(expr); self.emit(Instruction::Raise); }
            Stmt::Try { body, catch_body, finally_body, .. } => {
                let try_pos = self.current_pos();
                self.emit(Instruction::TryCatch(0, 0));
                for s in body { self.compile_stmt(s); }
                let jump_over_catch = self.current_pos();
                self.emit(Instruction::Jump(0));
                let catch_start = self.current_pos();
                for s in catch_body { self.compile_stmt(s); }
                let finally_start = self.current_pos();
                for s in finally_body { self.compile_stmt(s); }
                self.emit_at(try_pos, Instruction::TryCatch(
                    (catch_start - try_pos) as i32,
                    (finally_start - catch_start) as i32,
                ));
                self.emit_at(jump_over_catch, Instruction::Jump((finally_start - jump_over_catch) as i32));
                self.emit(Instruction::EndTry);
            }
            Stmt::Expression(expr) => { self.compile_expr(expr); self.emit(Instruction::Pop); }
            Stmt::AddToList { element, list } => {
                self.compile_expr(element);
                let l = self.s(list);
                self.emit(Instruction::LoadVar(l));
                self.emit(Instruction::AddToList);
                let l2 = self.s(list);
                self.emit(Instruction::StoreVar(l2));
            }
            Stmt::RemoveFromList { element, list } => {
                self.compile_expr(element);
                let l = self.s(list);
                self.emit(Instruction::LoadVar(l));
                self.emit(Instruction::RemoveFromList);
                let l2 = self.s(list);
                self.emit(Instruction::StoreVar(l2));
            }
            Stmt::Convert { expr, target_type, var } => {
                self.compile_expr(expr);
                let t = self.s(target_type);
                self.emit(Instruction::Convert(t));
                if !var.is_empty() {
                    let v = self.s(var);
                    self.emit(Instruction::StoreVar(v));
                }
            }
            Stmt::FuncDef { name, params, body } => {
                let func_idx = self.func_entries.len() as u32;
                let mut fc = Compiler::new();
                for s in body { fc.compile_stmt(s); }
                let n = fc.null_idx();
                fc.emit(Instruction::Const(n));
                fc.emit(Instruction::Return);
                let entry = self.absorb_func_compiler(fc, name, params, &[]);
                self.func_entries.push(entry);
                let n_idx = self.s(name);
                self.emit(Instruction::MakeFunc(func_idx));
                self.emit(Instruction::StoreVar(n_idx));
            }
            Stmt::FuncCall { func, args, result_var } => {
                if let Expr::Call { .. } = func {
                    self.compile_expr(func);
                } else {
                    for arg in args { self.compile_expr(arg); }
                    self.compile_expr(func);
                    self.emit(Instruction::Call(args.len() as u32));
                }
                if let Some(var) = result_var {
                    let v = self.s(var);
                    self.emit(Instruction::StoreVar(v));
                } else {
                    self.emit(Instruction::Pop);
                }
            }
            Stmt::ClassDef { name, parent, implements, fields, constructor, destructor, methods, publics, .. } => {
                let class_idx = self.class_entries.len();

                let mut compiled_methods = Vec::new();
                if let Some(ctor) = constructor {
                    let mut cc = Compiler::new();
                    for s in &ctor.body { cc.compile_stmt(s); }
                    let n = cc.null_idx();
                    cc.emit(Instruction::Const(n));
                    cc.emit(Instruction::Return);
                    let entry = self.absorb_func_compiler(cc, &ctor.name, &ctor.params, &[]);
                    compiled_methods.push(entry);
                }
                if let Some(dt) = destructor {
                    let mut dc = Compiler::new();
                    for s in &dt.body { dc.compile_stmt(s); }
                    let n = dc.null_idx();
                    dc.emit(Instruction::Const(n));
                    dc.emit(Instruction::Return);
                    let entry = self.absorb_func_compiler(dc, &dt.name, &[], &[]);
                    compiled_methods.push(entry);
                }
                for m in methods {
                    let mut mc = Compiler::new();
                    for s in &m.body { mc.compile_stmt(s); }
                    let n = mc.null_idx();
                    mc.emit(Instruction::Const(n));
                    mc.emit(Instruction::Return);
                    let entry = self.absorb_func_compiler(mc, &m.name, &m.params, &[]);
                    compiled_methods.push(entry);
                }

                let mut field_values: Vec<(String, Value)> = fields.iter()
                    .map(|(n, _)| (n.clone(), Value::Null))
                    .collect();

                if let Some(ref parent_name) = parent {
                    if let Some(pf) = self.class_fields.iter().find(|(n, _)| n == parent_name) {
                        for (pf_name, pf_val) in &pf.1 {
                            if !field_values.iter().any(|(n, _)| n == pf_name) {
                                field_values.push((pf_name.clone(), pf_val.clone()));
                            }
                        }
                    }
                    let pname_idx = self.s(parent_name);

                    let mut inherited_entries: Vec<FuncEntry> = Vec::new();
                    if let Some(parent_entry) = self.class_entries.iter().find(|ce| ce.name_idx == pname_idx) {
                        for pm_entry in &parent_entry.methods {
                            inherited_entries.push(pm_entry.clone());
                        }
                    }
                    for pm_entry in &inherited_entries {
                        let pname = pm_entry.resolve(&self.constants).name;
                        if !compiled_methods.iter().any(|m| {
                            let mname = m.resolve(&self.constants).name;
                            mname == pname
                        }) {
                            compiled_methods.push(pm_entry.clone());
                        } else {
                            let super_name = format!("__super_{}", pname);
                            if !compiled_methods.iter().any(|m| {
                                let mname = m.resolve(&self.constants).name;
                                mname == super_name
                            }) {
                                let mut super_entry = pm_entry.clone();
                                let s_idx = self.s(&super_name);
                                super_entry.name_idx = s_idx;
                                compiled_methods.push(super_entry);
                            }
                        }
                    }
                    if constructor.is_some() {
                        let pname_idx2 = self.s(parent_name);
                        let create_idx = self.s("create");
                        if let Some(parent_entry) = self.class_entries.iter().find(|ce| ce.name_idx == pname_idx2) {
                            let parent_ctor = parent_entry.methods.iter().find(|m| m.name_idx == create_idx);
                            if let Some(pctor) = parent_ctor {
                                let resolved = pctor.resolve(&self.constants);
                                if resolved.params.is_empty() {
                                    let ccreate_idx = self.s("create");
                                    if let Some(ctor_idx) = compiled_methods.iter().position(|m| m.name_idx == ccreate_idx) {
                                        let self_idx = self.s("self");
                                        let sup_idx = self.s("__super_create");
                                        let mut new_code = vec![
                                            Instruction::LoadVar(self_idx),
                                            Instruction::LoadMethod(sup_idx),
                                            Instruction::Call(1),
                                            Instruction::Pop,
                                        ];
                                        new_code.extend(compiled_methods[ctor_idx].code.clone());
                                        compiled_methods[ctor_idx].code = new_code;
                                    }
                                }
                            }
                        }
                    }
                    if destructor.is_some() {
                        let pname_idx3 = self.s(parent_name);
                        let destroy_idx = self.s("destroy");
                        if let Some(parent_entry) = self.class_entries.iter().find(|ce| ce.name_idx == pname_idx3) {
                            if parent_entry.methods.iter().any(|m| m.name_idx == destroy_idx) {
                                let ddestroy_idx = self.s("destroy");
                                if let Some(dtor_idx) = compiled_methods.iter().position(|m| m.name_idx == ddestroy_idx) {
                                    let code_len = compiled_methods[dtor_idx].code.len();
                                    if code_len >= 2 {
                                        let idx_pos = code_len - 2;
                                        let self_idx = self.s("self");
                                        let sup_idx = self.s("__super_destroy");
                                        let super_call = vec![
                                            Instruction::LoadVar(self_idx),
                                            Instruction::LoadMethod(sup_idx),
                                            Instruction::Call(1),
                                            Instruction::Pop,
                                        ];
                                        for (i, inst) in super_call.into_iter().enumerate() {
                                            compiled_methods[dtor_idx].code.insert(idx_pos + i, inst);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                for iface_name in implements {
                    if let Some(iface) = self.interface_table.iter().find(|(n, _)| n == iface_name) {
                        for im in &iface.1 {
                            let found = compiled_methods.iter().any(|m| {
                                let resolved = m.resolve(&self.constants);
                                resolved.name == im.name && resolved.params.len() == im.params.len()
                            });
                            if !found {
                                panic!("class {} does not implement interface {}: method {}({}) not found",
                                    name, iface_name, im.name, im.params.join(", "));
                            }
                        }
                    } else {
                        panic!("interface {} not found", iface_name);
                    }
                }

                let field_name_idxs: Vec<u32> = field_values.iter().map(|(n, _)| self.s(n)).collect();
                let public_idxs: Vec<u32> = publics.iter().map(|p| self.s(p)).collect();
                let ctor_params: Vec<u32> = if let Some(ctor) = &constructor {
                    ctor.params.iter().map(|p| self.s(p)).collect()
                } else { vec![] };

                let n_idx = self.s(name);
                self.class_entries.push(ClassEntry {
                    name_idx: n_idx,
                    field_names: field_name_idxs,
                    methods: compiled_methods.clone(),
                    publics: public_idxs,
                    constructor_params: ctor_params,
                });
                self.class_fields.push((name.clone(), field_values.clone()));

                let n_idx2 = self.s(name);
                self.emit(Instruction::MakeClass(class_idx as u32));
                self.emit(Instruction::StoreVar(n_idx2));
            }
            Stmt::Instantiate { class_name, args, var } => {
                for arg in args { self.compile_expr(arg); }
                let c = self.s(class_name);
                self.emit(Instruction::Instantiate(c));
                if let Some(v) = var {
                    let v_idx = self.s(v);
                    self.emit(Instruction::StoreVar(v_idx));
                } else {
                    self.emit(Instruction::Pop);
                }
            }
            Stmt::Chapter { stmts, .. } => { for s in stmts { self.compile_stmt(s); } }
            Stmt::InterfaceDef { name, methods, extends } => {
                let mut all_methods = methods.clone();
                for parent_name in extends {
                    if let Some((_, parent_methods)) = self.interface_table.iter().find(|(n, _)| n == parent_name) {
                        for pm in parent_methods {
                            if !all_methods.iter().any(|m| m.name == pm.name) {
                                all_methods.push(pm.clone());
                            }
                        }
                    } else {
                        panic!("interface {} extends unknown interface {}", name, parent_name);
                    }
                }
                self.interface_table.push((name.clone(), all_methods));
            }
            Stmt::Refer { module, symbols: _ } => {
                let mod_path = self.resolve_module_path(module);
                let bc_path = if mod_path.ends_with(".ls") {
                    mod_path[..mod_path.len() - 3].to_string() + ".lsbc"
                } else {
                    format!("{}.lsbc", mod_path)
                };

                let loaded = if std::path::Path::new(&bc_path).exists() {
                    match std::fs::read(&bc_path) {
                        Ok(bytes) => {
                            let module = BytecodeModule::decode(&bytes);
                            self.merge_module(module);
                            true
                        }
                        Err(_) => false,
                    }
                } else { false };

                if !loaded {
                    let source = match std::fs::read_to_string(&mod_path) {
                        Ok(s) => s,
                        Err(_) => { panic!("cannot read module: {}", mod_path); }
                    };
                    let tokens = {
                        let mut lexer = crate::lexer::Lexer::new(&source);
                        let mut tokens = Vec::new();
                        loop {
                            let tok = lexer.next_token();
                            if tok == crate::lexer::Token::EOF { break; }
                            if matches!(tok, crate::lexer::Token::Illegal(_)) { break; }
                            tokens.push(tok);
                        }
                        tokens
                    };
                    let program = {
                        let mut parser = crate::parser::Parser::new(tokens);
                        parser.parse_program()
                    };

                    let mod_dir = std::path::Path::new(&mod_path).parent()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let mut mc = if mod_dir.is_empty() {
                        Compiler::new()
                    } else {
                        Compiler::with_source_dir(mod_dir)
                    };
                    for s in &program.stmts { mc.compile_stmt(s); }
                    mc.emit(Instruction::Halt);
                    let module = BytecodeModule {
                        constants: mc.constants,
                        func_entries: mc.func_entries,
                        class_entries: mc.class_entries,
                        main_code: mc.code,
                    };
                    let _ = std::fs::write(&bc_path, module.encode());
                    self.merge_module(module);
                }
            }
        }
    }

    fn merge_module(&mut self, module: BytecodeModule) {
        let const_mapping: Vec<u32> = module.constants.items.iter().map(|entry| {
            match entry {
                PoolEntry::String(s) => self.constants.add_string(s),
                PoolEntry::Number(n) => self.constants.add_number(*n),
                PoolEntry::Bool(b) => self.constants.add_bool(*b),
                PoolEntry::Null => self.constants.add_null(),
            }
        }).collect();

        let func_offset = self.func_entries.len() as u32;
        let class_offset = self.class_entries.len() as u32;

        for fe in &module.func_entries {
            let remapped_code = Self::remap_instructions(&fe.code, &const_mapping, func_offset, class_offset);
            self.func_entries.push(FuncEntry {
                name_idx: const_mapping[fe.name_idx as usize],
                params: fe.params.iter().map(|&i| const_mapping[i as usize]).collect(),
                captures: fe.captures.iter().map(|&i| const_mapping[i as usize]).collect(),
                code: remapped_code,
            });
        }

        for ce in &module.class_entries {
            let mut new_methods = Vec::new();
            for m in &ce.methods {
                let remapped_code = Self::remap_instructions(&m.code, &const_mapping, func_offset, class_offset);
                new_methods.push(FuncEntry {
                    name_idx: const_mapping[m.name_idx as usize],
                    params: m.params.iter().map(|&i| const_mapping[i as usize]).collect(),
                    captures: m.captures.iter().map(|&i| const_mapping[i as usize]).collect(),
                    code: remapped_code,
                });
            }
            self.class_entries.push(ClassEntry {
                name_idx: const_mapping[ce.name_idx as usize],
                field_names: ce.field_names.iter().map(|&i| const_mapping[i as usize]).collect(),
                methods: new_methods,
                publics: ce.publics.iter().map(|&i| const_mapping[i as usize]).collect(),
                constructor_params: ce.constructor_params.iter().map(|&i| const_mapping[i as usize]).collect(),
            });
        }

        let new_code = Self::remap_instructions(&module.main_code, &const_mapping, func_offset, class_offset);
        self.code.extend(new_code);
    }

    fn remap_instructions(insts: &[Instruction], const_map: &[u32], func_offset: u32, class_offset: u32) -> Vec<Instruction> {
        insts.iter().map(|inst| {
            match inst {
                Instruction::Const(idx) => Instruction::Const(const_map[*idx as usize]),
                Instruction::LoadVar(idx) => Instruction::LoadVar(const_map[*idx as usize]),
                Instruction::StoreVar(idx) => Instruction::StoreVar(const_map[*idx as usize]),
                Instruction::LoadLocal(_) => inst.clone(),
                Instruction::StoreLocal(_) => inst.clone(),
                Instruction::MakeFunc(idx) => Instruction::MakeFunc(idx + func_offset),
                Instruction::MakeClosure(idx, caps) => {
                    let new_caps: Vec<u32> = caps.iter().map(|&i| const_map[i as usize]).collect();
                    Instruction::MakeClosure(idx + func_offset, new_caps)
                }
                Instruction::MakeClass(idx) => Instruction::MakeClass(idx + class_offset),
                Instruction::Instantiate(idx) => Instruction::Instantiate(const_map[*idx as usize]),
                Instruction::LoadMethod(idx) => Instruction::LoadMethod(const_map[*idx as usize]),
                Instruction::Ask(idx) => Instruction::Ask(const_map[*idx as usize]),
                Instruction::ReadFile(idx) => Instruction::ReadFile(const_map[*idx as usize]),
                Instruction::Convert(idx) => Instruction::Convert(const_map[*idx as usize]),
                _ => inst.clone(),
            }
        }).collect()
    }

    pub fn compile(mut self, program: &Program) -> BytecodeModule {
        let has_start = program.stmts.iter().any(|s| matches!(s, Stmt::StartHere(_)));
        if has_start {
            for stmt in &program.stmts {
                if let Stmt::StartHere(body) = stmt {
                    for s in body { self.compile_stmt(s); }
                }
            }
        } else {
            for stmt in &program.stmts {
                self.compile_stmt(stmt);
            }
        }
        self.emit(Instruction::Halt);
        BytecodeModule {
            constants: self.constants,
            func_entries: self.func_entries,
            class_entries: self.class_entries,
            main_code: self.code,
        }
    }
}
