" QBHD/QB64 BASIC syntax highlighting
if exists("b:current_syntax")
  finish
endif

syn case ignore

" Keywords
syn keyword basicKeyword PRINT DIM IF THEN ELSE ELSEIF END FOR NEXT WHILE WEND DO LOOP
syn keyword basicKeyword SUB FUNCTION RETURN CALL GOTO GOSUB ON ERROR RESUME
syn keyword basicKeyword SELECT CASE INPUT LINE CLS SCREEN COLOR LOCATE
syn keyword basicKeyword OPEN CLOSE READ WRITE DATA RESTORE
syn keyword basicKeyword AND OR NOT XOR MOD

" QB64 specific
syn keyword basicKeyword _RGB _RGBA _NEWIMAGE _LOADIMAGE _FREEIMAGE _PUTIMAGE
syn keyword basicKeyword _DISPLAY _LIMIT _KEYHIT _KEYDOWN _MOUSEINPUT
syn keyword basicKeyword _SCREENWIDTH _SCREENHEIGHT _DEST _SOURCE

" Types
syn keyword basicType INTEGER LONG SINGLE DOUBLE STRING AS

" Built-in functions
syn keyword basicFunction ABS ATN COS SIN TAN EXP LOG SQR INT RND
syn keyword basicFunction LEFT$ RIGHT$ MID$ LEN CHR$ ASC STR$ VAL
syn keyword basicFunction INKEY$ TIMER POINT PMAP

" Numbers
syn match basicNumber "\<\d\+\>"
syn match basicNumber "\<\d\+\.\d*\>"

" Strings
syn region basicString start=+"+ end=+"+

" Comments
syn match basicComment "'.*$"
syn keyword basicTodo TODO FIXME XXX NOTE contained
syn match basicComment "REM.*$" contains=basicTodo

" Operators
syn match basicOperator "[+\-*/=<>]"

hi def link basicKeyword Keyword
hi def link basicType Type
hi def link basicFunction Function
hi def link basicNumber Number
hi def link basicString String
hi def link basicComment Comment
hi def link basicTodo Todo
hi def link basicOperator Operator

let b:current_syntax = "basic"
