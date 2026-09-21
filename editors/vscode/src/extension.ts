import * as path from 'path';
import * as fs from 'fs';
import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';
import { AuraHoverProvider } from './hover';
import { AuraDebugConfigurationProvider } from './debug';

let client: LanguageClient | undefined;
let statusBarItem: vscode.StatusBarItem;
let terminal: vscode.Terminal | undefined;

function getAuraTerminal(): vscode.Terminal {
    if (!terminal || terminal.exitStatus !== undefined) {
        terminal = vscode.window.createTerminal('Aura');
    }
    return terminal;
}

function resolveBinary(binaryName: string, configPathKey: string): string {
    const config = vscode.workspace.getConfiguration('aura');
    const customPath = config.get<string>(configPathKey);
    if (customPath && customPath.trim() !== '') {
        return customPath.trim();
    }

    // Check workspace target/release or target/debug
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders) {
        for (const folder of workspaceFolders) {
            const releaseBin = path.join(folder.uri.fsPath, 'target', 'release', binaryName);
            if (fs.existsSync(releaseBin)) {
                return releaseBin;
            }
            const debugBin = path.join(folder.uri.fsPath, 'target', 'debug', binaryName);
            if (fs.existsSync(debugBin)) {
                return debugBin;
            }
        }
    }

    return binaryName;
}

export function activate(context: vscode.ExtensionContext) {
    const serverBinary = resolveBinary('auralsp', 'server.path');

    const serverOptions: ServerOptions = {
        command: serverBinary,
        args: [],
        transport: TransportKind.stdio
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'aura' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.aura')
        }
    };

    client = new LanguageClient(
        'auraLanguageServer',
        'Aura Language Server',
        serverOptions,
        clientOptions
    );

    client.start().then(() => {
        updateStatusBar('ready');
    }).catch((err) => {
        console.error('Failed to start Aura LSP:', err);
        updateStatusBar('error');
    });

    // Create Status Bar Item
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.command = 'aura.showMenu';
    context.subscriptions.push(statusBarItem);
    updateStatusBar('starting');
    statusBarItem.show();

    // Register Rich Contextual Hover Provider for Aura keywords, types, and operators
    context.subscriptions.push(
        vscode.languages.registerHoverProvider('aura', new AuraHoverProvider())
    );

    // Register Aura Debug Configuration Provider
    context.subscriptions.push(
        vscode.debug.registerDebugConfigurationProvider('aura', new AuraDebugConfigurationProvider(resolveBinary))
    );

    // Register Commands
    context.subscriptions.push(
        vscode.commands.registerCommand('aura.showMenu', async () => {
            const selected = await vscode.window.showQuickPick([
                { label: '$(play) Run Active File', description: 'Compile and run active .aura file', id: 'run' },
                { label: '$(debug-alt) Debug Active File', description: 'Debug active .aura file with breakpoints & sourcemaps', id: 'debug' },
                { label: '$(debug-step-into) Step Debug Active File (CLI)', description: 'Interactive step-by-step debugger in terminal', id: 'stepDebug' },
                { label: '$(check) Type Check Active File', description: 'Run aurac check on active file', id: 'check' },
                { label: '$(gear) Build Standalone Binary', description: 'Compile to standalone native binary', id: 'build' },
                { label: '$(beaker) Run Project Tests', description: 'Execute test suite with auratest', id: 'test' },
                { label: '$(edit) Format Active File', description: 'Format using aurafmt', id: 'fmt' },
                { label: '$(eye) Start Live Watch Mode', description: 'Watch and recompile on file save', id: 'watch' },
                { label: '$(browser) Open Online Playground', description: 'Open local interactive playground', id: 'playground' },
                { label: '$(refresh) Restart Language Server', description: 'Restart Aura LSP daemon', id: 'restart' }
            ], {
                placeHolder: 'Aura Language Commands'
            });

            if (!selected) return;

            switch (selected.id) {
                case 'run':
                    vscode.commands.executeCommand('aura.runActiveFile');
                    break;
                case 'debug':
                    vscode.commands.executeCommand('aura.debugActiveFile');
                    break;
                case 'stepDebug':
                    vscode.commands.executeCommand('aura.stepDebugActiveFile');
                    break;
                case 'check':
                    vscode.commands.executeCommand('aura.checkActiveFile');
                    break;
                case 'build':
                    vscode.commands.executeCommand('aura.buildActiveFile');
                    break;
                case 'test':
                    vscode.commands.executeCommand('aura.runTests');
                    break;
                case 'fmt':
                    vscode.commands.executeCommand('aura.formatFile');
                    break;
                case 'watch':
                    vscode.commands.executeCommand('aura.startWatch');
                    break;
                case 'playground':
                    vscode.commands.executeCommand('aura.openPlayground');
                    break;
                case 'restart':
                    vscode.commands.executeCommand('aura.restartServer');
                    break;
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('aura.runActiveFile', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'aura') {
                vscode.window.showWarningMessage('Please open an Aura (.aura) file first.');
                return;
            }
            await editor.document.save();
            const aurac = resolveBinary('aurac', 'compiler.path');
            const term = getAuraTerminal();
            term.show();
            term.sendText(`${aurac} run "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('aura.debugActiveFile', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'aura') {
                vscode.window.showWarningMessage('Please open an Aura (.aura) file first.');
                return;
            }
            await editor.document.save();
            await vscode.debug.startDebugging(undefined, {
                type: 'aura',
                request: 'launch',
                name: `Debug Aura: ${path.basename(editor.document.fileName)}`,
                program: editor.document.fileName
            });
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('aura.stepDebugActiveFile', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'aura') {
                vscode.window.showWarningMessage('Please open an Aura (.aura) file first.');
                return;
            }
            await editor.document.save();
            const aurac = resolveBinary('aurac', 'compiler.path');
            const term = getAuraTerminal();
            term.show();
            term.sendText(`${aurac} step "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('aura.checkActiveFile', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'aura') {
                vscode.window.showWarningMessage('Please open an Aura (.aura) file first.');
                return;
            }
            await editor.document.save();
            const aurac = resolveBinary('aurac', 'compiler.path');
            const term = getAuraTerminal();
            term.show();
            term.sendText(`${aurac} check "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('aura.buildActiveFile', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'aura') {
                vscode.window.showWarningMessage('Please open an Aura (.aura) file first.');
                return;
            }
            await editor.document.save();
            const aurac = resolveBinary('aurac', 'compiler.path');
            const term = getAuraTerminal();
            term.show();
            term.sendText(`${aurac} build "${editor.document.fileName}" --standalone`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('aura.runTests', async () => {
            const auratest = resolveBinary('auratest', 'compiler.path');
            const term = getAuraTerminal();
            term.show();
            term.sendText(`${auratest}`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('aura.formatFile', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'aura') {
                vscode.window.showWarningMessage('Please open an Aura (.aura) file first.');
                return;
            }
            await editor.document.save();
            const aurafmt = resolveBinary('aurafmt', 'compiler.path');
            const term = getAuraTerminal();
            term.show();
            term.sendText(`${aurafmt} -w "${editor.document.fileName}"`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('aura.startWatch', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor || editor.document.languageId !== 'aura') {
                vscode.window.showWarningMessage('Please open an Aura (.aura) file first.');
                return;
            }
            const aurac = resolveBinary('aurac', 'compiler.path');
            const term = getAuraTerminal();
            term.show();
            term.sendText(`${aurac} watch "${editor.document.fileName}" --run`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('aura.restartServer', async () => {
            if (client) {
                updateStatusBar('starting');
                await client.stop();
                await client.start();
                updateStatusBar('ready');
                vscode.window.showInformationMessage('Aura Language Server restarted.');
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('aura.openPlayground', () => {
            vscode.env.openExternal(vscode.Uri.parse('http://127.0.0.1:3000'));
        })
    );
}

function updateStatusBar(status: 'starting' | 'ready' | 'error') {
    if (!statusBarItem) return;
    switch (status) {
        case 'starting':
            statusBarItem.text = '$(sync~spin) Aura';
            statusBarItem.tooltip = 'Aura Language Server is starting...';
            break;
        case 'ready':
            statusBarItem.text = '$(zap) Aura';
            statusBarItem.tooltip = 'Aura Language Server is ready (Click for menu)';
            break;
        case 'error':
            statusBarItem.text = '$(alert) Aura';
            statusBarItem.tooltip = 'Aura Language Server failed to start (Click to restart)';
            break;
    }
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
