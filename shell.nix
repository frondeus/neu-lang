with import <nixpkgs> {}; {
  neuLangEnv = stdenv.mkDerivation {
    name = "neu-lang-env";
    buildInputs = [ gcc git colordiff ];
  };
}

