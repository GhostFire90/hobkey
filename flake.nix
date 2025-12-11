{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          qemu
          gnumake
          (limine.override {
            enableAll = true;
            buildCDs = true;
            biosSupport = true;
          }).dev
          libisoburn
	  lldb
        ];

      };
    }
  );
}
