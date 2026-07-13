const vscode = require('vscode');

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
    console.log('LampScript extension is now active!');

    const provider = vscode.languages.registerCompletionItemProvider(
        'lampscript',
        {
            provideCompletionItems(document, position, token, context) {
                const completions = [];

                // Keywords
                const keywords = ['let', 'mod', 'fn', 'if', 'else', 'while', 'return', 'alloc'];
                for (const kw of keywords) {
                    const item = new vscode.CompletionItem(kw, vscode.CompletionItemKind.Keyword);
                    completions.push(item);
                }

                // Types
                const types = [
                    'u8', 'u16', 'u32', 'u64', 
                    'i8', 'i16', 'i32', 'i64', 
                    'f8', 'f16', 'f32', 'f64', 
                    'bool', 'char', 'string', 'void'
                ];
                for (const t of types) {
                    const item = new vscode.CompletionItem(t, vscode.CompletionItemKind.TypeParameter);
                    completions.push(item);
                }

                // Constants
                const constants = ['true', 'false'];
                for (const c of constants) {
                    const item = new vscode.CompletionItem(c, vscode.CompletionItemKind.Constant);
                    completions.push(item);
                }

                // Built-in Functions
                const printCompletion = new vscode.CompletionItem('print?', vscode.CompletionItemKind.Function);
                printCompletion.documentation = new vscode.MarkdownString('Print a message to the console without a newline.');
                printCompletion.insertText = new vscode.SnippetString('print?(${1:"message"});');
                completions.push(printCompletion);

                const printlnCompletion = new vscode.CompletionItem('println?', vscode.CompletionItemKind.Function);
                printlnCompletion.documentation = new vscode.MarkdownString('Print a message to the console with a newline.');
                printlnCompletion.insertText = new vscode.SnippetString('println?(${1:"message"});');
                completions.push(printlnCompletion);

                return completions;
            }
        }
    );

    context.subscriptions.push(provider);
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
