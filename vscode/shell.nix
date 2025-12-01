{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "avon-vscode-extension";

  buildInputs = with pkgs; [
    nodejs
    nodePackages.npm
  ];

  shellHook = ''
    echo "Avon VS Code Extension Development Environment"
    echo "================================================"
    echo ""
    echo "Available commands:"
    echo "  npm install        - Install dependencies (includes vsce)"
    echo "  npm run compile    - Compile TypeScript"
    echo "  npm run watch      - Watch and compile TypeScript"
    echo "  npm run package    - Build VSIX package"
    echo "  npx vsce package   - Build VSIX package (directly)"
    echo ""
    echo "To build the extension:"
    echo "  1. npm install"
    echo "  2. npm run compile"
    echo "  3. npm run package"
    echo ""
    echo "The VSIX file will be generated as: avon-0.1.0.vsix"
    echo ""
  '';
}
