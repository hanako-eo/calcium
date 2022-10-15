%{
  #include "grammar.h"
%}

%option noyywrap

%%
[0-9]+ { yylval.num = atof(yytext); return NUMBER; }
[0-9]+\.[0-9]* { yylval.num = atof(yytext); return NUMBER; }
\<\- { return LEFT_ARROW; }
\+ { return PLUS; }
\- { return MINUS; }
\* { return MULT; }
\/ { return DIV; }
\! { return FACTORIAL; }
\? { return SUMM; }
\^ { return POWER; }
\# { return ROOT; }
\( { return LEFT_P; }
\) { return RIGHT_P; }
\n { return EOL; }
%%
