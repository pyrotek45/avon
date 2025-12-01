import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';
import * as fs from 'fs';
import * as path from 'path';

let client: LanguageClient;

function log(message: string) {
    const logPath = '/tmp/avon_extension_debug.log';
    const timestamp = new Date().toISOString();
    fs.appendFileSync(logPath, `[${timestamp}] ${message}\n`);
}

export function activate(context: vscode.ExtensionContext) {
    log('Activate called');
    
    // Try multiple locations for the avon-lsp binary
    const possiblePaths = [
        path.join(context.extensionPath, 'avon-lsp'),     // Bundled binary
        '/usr/local/bin/avon-lsp',                        // System-wide install
        '/usr/bin/avon-lsp',                              // Alternative system path
        path.join(process.env.HOME || '', '.cargo/bin/avon-lsp'),  // Cargo install
    ];
    
    let serverCommand: string | null = null;
    for (const candidate of possiblePaths) {
        if (fs.existsSync(candidate)) {
            serverCommand = candidate;
            break;
        }
    }
    
    log(`Trying paths: ${possiblePaths.join(', ')}`);
    log(`Server command: ${serverCommand}`);
    
    if (!serverCommand) {
        log(`ERROR: Server binary not found in any location`);
        vscode.window.showErrorMessage(
            'Avon LSP binary not found. Install with: cargo install --path . --bin avon_lsp'
        );
        return;
    }

    const serverOptions: ServerOptions = {
        command: serverCommand,
        args: []
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'avon' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.av')
        },
        outputChannelName: 'Avon LSP Client'
    };

    log('Creating LanguageClient');
    client = new LanguageClient(
        'avon',
        'Avon Language Server',
        serverOptions,
        clientOptions
    );

    log('Starting client');
    client.start().then(() => {
        log('Client started successfully');
    }).catch(err => {
        log(`ERROR starting client: ${err}`);
        vscode.window.showErrorMessage(`Failed to start Avon LSP: ${err}`);
    });

    context.subscriptions.push(client);

    console.log('Avon Language Server activated');
    log('Activation complete');
}

export function deactivate(): Thenable<void> | undefined {
    log('Deactivate called');
    if (!client) {
        return undefined;
    }
    return client.stop();
}
