#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Identifier(String),
    BinaryOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    ListLiteral(Vec<Expr>),
    MapLiteral(Vec<(String, Expr)>),
    TypeOf(Box<Expr>),
    Capitalize(Box<Expr>),
    Input,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod, Pow,
    And, Or,
    Eq, Ne, Gt, Lt, Ge, Le,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg, Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDef {
        name: String,
        value: Expr,
    },
    VarDecl {
        name: String,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    Say(Expr),
    Ask {
        prompt: Expr,
        var: String,
    },
    ReadFile {
        filename: Expr,
        var: String,
    },
    WriteFile {
        content: Expr,
        filename: Expr,
    },
    If {
        condition: Expr,
        body: Vec<Stmt>,
        otherwise: Vec<Stmt>,
    },
    Repeat {
        times: Expr,
        body: Vec<Stmt>,
    },
    ForEach {
        var: String,
        collection: Expr,
        body: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    Stop,
    Exit(Option<Expr>),
    Raise(Expr),
    Try {
        body: Vec<Stmt>,
        catch_type: Option<String>,
        catch_body: Vec<Stmt>,
        finally_body: Vec<Stmt>,
    },
    Expression(Expr),
    Block(Vec<Stmt>),
    AddToList {
        element: Expr,
        list: String,
    },
    RemoveFromList {
        element: Expr,
        list: String,
    },
    Convert {
        expr: Expr,
        target_type: String,
        var: String,
    },
    FuncDef {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    FuncCall {
        func: Expr,
        args: Vec<Expr>,
        result_var: Option<String>,
    },
    ClassDef {
        name: String,
        parent: Option<String>,
        fields: Vec<(String, Expr)>,
        constructor: Option<ClassMethod>,
        destructor: Option<ClassMethod>,
        methods: Vec<ClassMethod>,
        publics: Vec<String>,
    },
    Instantiate {
        class_name: String,
        args: Vec<Expr>,
        var: Option<String>,
    },
    StartHere(Vec<Stmt>),
    Chapter {
        name: String,
        stmts: Vec<Stmt>,
    },
    Refer {
        module: String,
        symbols: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub struct ClassMethod {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

pub struct Program {
    pub stmts: Vec<Stmt>,
}
