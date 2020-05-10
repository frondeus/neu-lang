import * as monaco from 'monaco-editor';

const editor = monaco.editor.create(document.getElementById('container'), {
    language: 'neu'
});

const model = editor.getModel();
let diagnostics = [];

function write_diagnostic(json) {
    let line = json.line;
    let end_line = json.end_line;
    let col = json.col;
    let end_col = json.end_col;
    let diagnostic = {
        startLineNumber: line,
        endLineNumber: end_line,
        startColumn: col,
        endColumn: end_col,
        message: json.text,
        severity: json.severity
    }
    diagnostics.push(diagnostic);
    monaco.editor.setModelMarkers(model, 'neu', diagnostics);
}
function write_eval(e) {
    editor.addCommand(0, function() {
        alert('foo!');
    }, '')
    //monaco.languages.registerI//
    /*
    monaco.languages.registerCodeLensProvider('neu', {
        provideCodeLenses(model, token) {
            return {
                lenses: [
                    {
                        range: {
                            startLineNumber: 1,
                            startColumn: 2,
                            endLineNumber: 1,
                            endColumn: 1
                        },
                        id: "Eval",
                        command: {
                            id: 0,
                            title: `${e}`
                        }
                    }
                ]
            }
        },
        resolveCodeLens(model, codeLens, token) {
            return codeLens;
        }
    })
     */

    let output = document.getElementById('output');
    output.innerText = e;
}
function clear_diagnostics() {
    diagnostics = [];
    monaco.editor.setModelMarkers(model, 'neu', []);
}

window.neu = { write_diagnostic, clear_diagnostics, write_eval };

import('../pkg/index.js').then(neu_wasm => {
    model.onDidChangeContent(() => {
        const value = model.getValue();
        neu_wasm.on_change(value);
    });
})
