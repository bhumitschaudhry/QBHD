import { useRef, useCallback } from 'react';
import MonacoEditor from '@monaco-editor/react';

// BASIC language definition for Monaco
const BASIC_LANG = {
  defaultToken: '',
  keywords: [
    'PRINT', 'DIM', 'IF', 'THEN', 'ELSE', 'ELSEIF', 'END', 'FOR', 'NEXT',
    'WHILE', 'WEND', 'DO', 'LOOP', 'UNTIL', 'SELECT', 'CASE',
    'SUB', 'FUNCTION', 'RETURN', 'CALL', 'GOTO', 'GOSUB', 'ON', 'ERROR', 'RESUME',
    'INPUT', 'LINE', 'CLS', 'SCREEN', 'COLOR', 'LOCATE',
    'OPEN', 'CLOSE', 'READ', 'WRITE', 'DATA', 'RESTORE',
    'AND', 'OR', 'NOT', 'XOR', 'MOD',
    'AS', 'INTEGER', 'LONG', 'SINGLE', 'DOUBLE', 'STRING',
    'SHARED', 'STATIC', 'REDIM', 'PRESERVE',
    'EXIT', 'DEF', 'TYPE', 'DECLARE',
    'LET', 'STEP', 'TO', 'IS', 'TAB', 'SPC',
  ],
  qb64Keywords: [
    '_RGB', '_RGBA', '_NEWIMAGE', '_LOADIMAGE', '_FREEIMAGE', '_PUTIMAGE',
    '_DISPLAY', '_LIMIT', '_KEYHIT', '_KEYDOWN', '_MOUSEINPUT',
    '_SCREENWIDTH', '_SCREENHEIGHT', '_DEST', '_SOURCE',
    '_PRINTSTRING', '_PRINTMODE', '_FONT', '_LOADFONT',
    '_SNDOPEN', '_SNDPLAY', '_SNDCLOSE', '_SNDVOL',
    '_MOUSEX', '_MOUSEY', '_MOUSEBUTTON', '_MOUSEWHEEL',
    '_CEIL', '_ROUND', '_MAPUNICODE',
    '_D2R', '_R2D', '_PI',
  ],
  builtins: [
    'ABS', 'ATN', 'COS', 'SIN', 'TAN', 'EXP', 'LOG', 'SQR', 'INT', 'RND',
    'LEFT$', 'RIGHT$', 'MID$', 'LEN', 'CHR$', 'ASC', 'STR$', 'VAL',
    'INKEY$', 'TIMER', 'POINT', 'PMAP', 'INP', 'OUT',
    'PEEK', 'POKE', 'ENVIRON$', 'ENVIRON',
    'DATE$', 'TIME$', 'NOW',
    'EOF', 'LOF', 'FREEFILE', 'SEEG',
    'LBOUND', 'UBOUND', 'SIZE',
  ],
  operators: ['=', '<>', '<', '>', '<=', '>=', '+', '-', '*', '/', '\\', '^', '&'],
  symbols: /[=><!~?:&|+\-*\/\^%]+/,

  tokenizer: {
    root: [
      // Comments - single quote
      [/'.*$/, 'comment'],
      // Comments - REM
      [/\bREM\b.*$/i, 'comment'],

      // String literals
      [/"([^"]|"")*"/, 'string'],

      // QB64 metacommands
      [/\$[A-Z]+/, 'meta'],

      // Labels (line starting with identifier followed by colon, not in a statement)
      [/^\w+:/, 'type.identifier'],

      // Numbers
      [/\d+\.\d*([eE][\-+]?\d+)?/, 'number.float'],
      [/\d+[eE][\-+]?\d+/, 'number.float'],
      [/\d+/, 'number'],

      // Hex and octal
      [/&[hH][0-9a-fA-F]+/, 'number.hex'],
      [/&[oO][0-7]+/, 'number.octal'],

      // Type suffixes
      [/[a-zA-Z_]\w*[$%!#&]/, {
        cases: {
          '@keywords': 'keyword',
          '@default': 'identifier'
        }
      }],

      // QB64 keywords (underscore prefix)
      [/[a-zA-Z_]\w*/, {
        cases: {
          '@qb64Keywords': 'keyword.underscore',
          '@keywords': 'keyword',
          '@builtins': 'predefined',
          '@default': 'identifier'
        }
      }],

      // Operators
      [/[<>]=?|<>|\*\*|[+\-*\/\\^=&]/, 'operator'],

      // Delimiters
      [/[;,.()\[\]]/, 'delimiter'],
    ],
  },
};

const BASIC_THEME = {
  base: 'vs-dark',
  inherit: true,
  rules: [
    { token: 'comment', foreground: '6A9955', fontStyle: 'italic' },
    { token: 'string', foreground: 'CE9178' },
    { token: 'number', foreground: 'B5CEA8' },
    { token: 'number.float', foreground: 'B5CEA8' },
    { token: 'number.hex', foreground: 'B5CEA8' },
    { token: 'number.octal', foreground: 'B5CEA8' },
    { token: 'keyword', foreground: '569CD6', fontStyle: 'bold' },
    { token: 'keyword.underscore', foreground: 'C586C0', fontStyle: 'bold' },
    { token: 'predefined', foreground: 'DCDCAA' },
    { token: 'identifier', foreground: '9CDCFE' },
    { token: 'operator', foreground: 'D4D4D4' },
    { token: 'delimiter', foreground: 'D4D4D4' },
    { token: 'meta', foreground: 'C586C0' },
    { token: 'type.identifier', foreground: '4EC9B0' },
  ],
  colors: {
    'editor.background': '#1e1e1e',
    'editor.foreground': '#d4d4d4',
    'editor.lineHighlightBackground': '#2a2d2e',
    'editor.selectionBackground': '#264f78',
    'editor.inactiveSelectionBackground': '#3a3d41',
  },
};

export default function Editor({ code, setCode, onCursorChange, onSave }) {
  const editorRef = useRef(null);

  const handleEditorDidMount = useCallback((editor, monaco) => {
    editorRef.current = editor;

    // Register BASIC language
    monaco.languages.register({ id: 'basic' });
    monaco.languages.setMonarchTokensProvider('basic', BASIC_LANG);
    monaco.editor.defineTheme('basic-dark', BASIC_THEME);
    monaco.editor.setTheme('basic-dark');

    // Track cursor position
    editor.onDidChangeCursorPosition((e) => {
      if (onCursorChange) {
        onCursorChange({
          line: e.position.lineNumber,
          column: e.position.column,
        });
      }
    });

    // Ctrl+S save
    editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
      if (onSave) onSave();
    });
  }, [onCursorChange, onSave]);

  return (
    <div className="editor">
      <MonacoEditor
        height="100%"
        language="basic"
        theme="basic-dark"
        value={code}
        onChange={(value) => setCode(value || '')}
        onMount={handleEditorDidMount}
        options={{
          minimap: { enabled: true },
          fontSize: 14,
          fontFamily: "'Consolas', 'Courier New', monospace",
          lineNumbers: 'on',
          roundedSelection: true,
          scrollBeyondLastLine: false,
          automaticLayout: true,
          wordWrap: 'on',
          tabSize: 4,
          insertSpaces: false,
          renderWhitespace: 'selection',
          bracketPairColorization: { enabled: true },
          smoothScrolling: true,
          cursorBlinking: 'smooth',
          cursorSmoothCaretAnimation: 'on',
        }}
      />
    </div>
  );
}
