" QBHD/QB64 BASIC syntax highlighting
if exists("b:current_syntax")
  finish
endif

syn case ignore

" Comments
syn match basicComment "'.*$"
syn match basicComment "^\s*REM\b.*$" contains=basicTodo
syn keyword basicTodo TODO FIXME XXX NOTE HACK contained

" Strings
syn region basicString start=+"+ end=+"+ contains=basicStringEscape
syn match basicStringEscape '""' contained

" Numbers
syn match basicNumber "\<\d\+\>"
syn match basicNumber "\<\d\+\.\d*\>"
syn match basicNumber "\<\d\+\.\d*[eE][\-+]\?\d\+\>"
syn match basicNumber "\<\d\+[eE][\-+]\?\d\+\>"
syn match basicNumber "&[hH][0-9a-fA-F]\+"
syn match basicNumber "&[oO][0-7]\+"

" Type suffixes on identifiers
syn match basicTypeSuffix "\w\+\$"
syn match basicTypeSuffix "\w\+%"
syn match basicTypeSuffix "\w\+&"
syn match basicTypeSuffix "\w\+!"
syn match basicTypeSuffix "\w\+#"

" Keywords - Control Flow
syn keyword basicConditional IF THEN ELSE ELSEIF END IF
syn keyword basicRepeat FOR NEXT TO STEP WHILE WEND DO LOOP UNTIL
syn keyword basicBranch SELECT CASE IS
syn keyword basicLabel GOTO GOSUB RETURN
syn keyword basicException ERROR RESUME ON

" Keywords - Declarations
syn keyword basicStorage DIM REDIM PRESERVE SHARED STATIC AS
syn keyword basicTypeDecl INTEGER LONG SINGLE DOUBLE STRING TYPE
syn keyword basicDeclare DECLARE SUB FUNCTION CALL

" Keywords - I/O
syn keyword basicIO PRINT INPUT OPEN CLOSE READ WRITE DATA RESTORE
syn keyword basicIO LINE INPUT

" Keywords - Screen/Graphics
syn keyword basicScreen CLS SCREEN COLOR LOCATE WIDTH VIEW WINDOW
syn keyword basicGraphics CIRCLE LINE PSET PRESET PAINT DRAW GET PUT PCOPY

" Keywords - File
syn keyword basicFile FIELD LSET RSET EOF LOF FREEFILE

" Keywords - Operators
syn keyword basicOperator AND OR NOT XOR MOD IS LET

" Keywords - Program Control
syn keyword basicStatement END STOP SYSTEM RUN CONT CHAIN COMMON
syn keyword basicStatement ERASE RANDOMIZE SWAP BEEP SLEEP EXIT
syn keyword basicStatement DEF OPTION BASE EXPLICIT

" Keywords - Conversion
syn keyword basicConversion MKI$ MKS$ MKD$ MKL$ CVI CVS CVD CVL
syn keyword basicConversion CINT CSNG CDBL CLNG

" QB64 specific keywords
syn keyword qb64Keyword _RGB _RGBA _NEWIMAGE _LOADIMAGE _FREEIMAGE _PUTIMAGE
syn keyword qb64Keyword _DISPLAY _LIMIT _SCREENIMAGE
syn keyword qb64Keyword _KEYHIT _KEYDOWN _MOUSEINPUT
syn keyword qb64Keyword _MOUSEX _MOUSEY _MOUSEBUTTON _MOUSEWHEEL
syn keyword qb64Keyword _MOUSESHOW _MOUSEHIDE
syn keyword qb64Keyword _SCREENWIDTH _SCREENHEIGHT _DEST _SOURCE
syn keyword qb64Keyword _SCREENX _SCREENY _SCREENMOVE
syn keyword qb64Keyword _PRINTSTRING _PRINTMODE _FONT _LOADFONT
syn keyword qb64Keyword _SNDOPEN _SNDPLAY _SNDCLOSE _SNDVOL
syn keyword qb64Keyword _CEIL _ROUND _PI _D2R _R2D
syn keyword qb64Keyword _CLIPBOARD$ _TITLE _RESIZE
syn keyword qb64Keyword _MAPUNICODE _GLRENDER _DELAY
syn keyword qb64Keyword _MEM _MEMNEW _MEMFREE _MEMGET _MEMPUT _MEMFILL _MEMCOPY

" Built-in functions
syn keyword basicFunction ABS ATN COS SIN TAN EXP LOG SQR INT RND
syn keyword basicFunction LEFT$ RIGHT$ MID$ LEN CHR$ ASC STR$ VAL
syn keyword basicFunction INKEY$ TIMER POINT PMAP INP OUT PEEK
syn keyword basicFunction LBOUND UBOUND SIZE
syn keyword basicFunction DATE$ TIME$ NOW
syn keyword basicFunction ENVIRON$ ENVIRON

" Built-in constants
syn keyword basicConst TRUE FALSE NULL

" Labels (lines starting with identifier followed by colon)
syn match basicLabelDef "^\w\+:" contains=basicLabelName
syn match basicLabelName "\w\+" contained

" Metacommands ($INCLUDE, $IF, etc.)
syn match basicMeta "\$INCLUDE"
syn match basicMeta "\$IF"
syn match basicMeta "\$ELSE"
syn match basicMeta "\$END IF"

" Highlighting links
hi def link basicComment Comment
hi def link basicTodo Todo
hi def link basicString String
hi def link basicStringEscape SpecialChar
hi def link basicNumber Number
hi def link basicConditional Conditional
hi def link basicRepeat Repeat
hi def link basicBranch Conditional
hi def link basicLabel Special
hi def link basicLabelDef Label
hi def link basicLabelName Label
hi def link basicException Exception
hi def link basicStorage StorageClass
hi def link basicTypeDecl Type
hi def link basicDeclare Structure
hi def link basicIO Include
hi def link basicScreen Keyword
hi def link basicGraphics Keyword
hi def link basicFile Keyword
hi def link basicOperator Operator
hi def link basicStatement Statement
hi def link basicConversion Function
hi def link basicFunction Function
hi def link basicConst Constant
hi def link basicMeta PreProc
hi def link qb64Keyword Special

let b:current_syntax = "basic"
