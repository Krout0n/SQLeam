# SQLeam

### How to run
```
$ cargo run
```

### Grammar
```
<TableDef> ::= Table <TableName> '{' { <MemberName>: <Type>, } '}'
<Stmt> ::= <TableName>'.'<Method>(Arg {, Arg});
<Method> ::= insert | delete
<Arg> ::= <Number> | <StrLiteral>
<Number> ::= series of 01234...9
<StrLiteral> ::= "<Chars>"
<Chars> ::= <Char> | <Char> <Chars>
<Char> ::= any char except "
<TableName> ::= <Ident>
<MemberName> ::= <Ident>
<Type> ::= "int" | "string"
<Ident> ::= series of ABCD ... XYZ | series of abc...xyz
```
