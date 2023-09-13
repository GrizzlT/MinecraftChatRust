{
  description = "Multiversion Minecraft JSON chat";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
  let
    systems = [ "x86_64-linux" ];
    pkgsUnwrapped = system: import nixpkgs {
      inherit system;
      overlays = [ (import rust-overlay) ];
    };
  in
  {
    devShells = nixpkgs.lib.genAttrs systems (system: let pkgs = pkgsUnwrapped system; in
      {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            rust-analyzer
            graphviz
            cargo-expand
          ];
        };
      });
  };
}
