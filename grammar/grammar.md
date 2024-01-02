# shaba-lang grammar

// ???
program     : rule + ;
rule        : stmt + ;

stmt        : decl | expr ;

decl        : let_decl | fn_decl ;
let_decl    : 'let' IDENTIFIER '=' expr ;
fn_decl     : 'fn' IDENTIFIER '(' IDENTIFIER * ')' '{' expr '}' ;

expr        : if_expr | unit_expr | let_expr | literal | unary_expr ;
if_expr     : 'if' expression '{' expr '}' else_tail ? ;
else_tail   : 'else' [ 'if_expr' | '{' expr '}' ] ;
unit_expr   : '()' ;
unary_expr  : unary_op expr ;
binary_expr : expr binary_op expr ;

unary_op    : '-' | '!' ;
binary_op   : arith_op | bool_op | comp_op ;
arith_op    : '+' | '-' | '*' | '/' | '%' ;
bool_op     : '&&' | '||' ;
comp_op     : '!=' | '==' | '>' | '>=' | '<' | '<=' ;

literal         : int_literal | double_literal | bool_literal | str_literal ;
bool_literal    : 'true' | 'false' ;
int_literal     : 0-9 + ;

// currently the lexer doesn't preserve syntax trivia so this doesn't matter
comment         : line_comment ;
line_comment    : '//' non_eol ;
