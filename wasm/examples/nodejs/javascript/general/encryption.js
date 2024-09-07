const astrix = require('../../../../nodejs/astrix');

astrix.initConsolePanicHook();

(async () => {

    let encrypted = astrix.encryptXChaCha20Poly1305("my message", "my_password");
    console.log("encrypted:", encrypted);
    let decrypted = astrix.decryptXChaCha20Poly1305(encrypted, "my_password");
    console.log("decrypted:", decrypted);

})();
