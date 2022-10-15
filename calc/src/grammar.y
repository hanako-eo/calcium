%{
  #include <math.h>
%}

%union {
  double num;
}

%token<num> NUMBER

%token PLUS
%token MINUS
%token POWER
%token ROOT
%token MULT
%token DIV
%token FACTORIAL
%token SUMM
%token LEFT_P
%token RIGHT_P
%token LEFT_ARROW

%token EOL;
%type<num> expr;
%type<num> calcul;

%left LEFT_ARROW
%left PLUS MINUS
%left DIV MULT
%left POWER ROOT
%left SUMM FACTORIAL

%%
main: | 
  main calcul;

calcul: expr |
  calcul EOL { $$ = $1; };

expr: 
  NUMBER { $$ = $1; } |
  MINUS expr { $$ = -$2; } |
  expr SUMM { $$ = ($1 * ($1 + 1))/2; } |
  expr PLUS expr { $$ = $1 + $3; } |
  expr MINUS expr { $$ = $1 - $3; } |
  expr FACTORIAL { $$ = tgamma($1 + 1); } |
  expr ROOT expr { $$ = pow($1, 1/$3); } |
  expr POWER expr { $$ = pow($1, $3); } |
  expr MULT expr { $$ = $1 * $3; } |
  expr DIV expr { $$ = $1 / $3; } |
  LEFT_P expr RIGHT_P { $$ = $2; };
%%

void execute_line(const char* line) {
  yy_scan_string(line);
  yyparse();
  printf("%f", yylval.num);
}

int yyerror(char* s){
  printf("YYERROR: %s\n", s);
  
  return 1;
}
