import('./tiny_alchemy_box')
    .then(wasm => {
        const renderButton = document.getElementById('render');
        const helpButton = document.getElementById('help');
        const closeHelpButton = document.getElementById('close_help');
        const helpText = document.getElementById('help_text');
        const scriptBox = document.getElementById('tab_script');
        const positionIndicator = document.getElementById('cursor_position');

        renderButton.addEventListener('click', () => {
            wasm.process(scriptBox.value);
        });

        helpButton.addEventListener('click', () => {
            helpText.hidden = false;
        });

        closeHelpButton.addEventListener('click', () => {
            helpText.hidden = true;
        });

        const boxEffects = ['keydown', 'keyup', 'keypress', 'click', 'mouseover', 'mousedown'];

        boxEffects.forEach((effect) => {
            scriptBox.addEventListener(effect, () => {
                const startPosition = scriptBox.selectionStart;
                positionIndicator.innerHTML = `Cursor position: ${startPosition}`
            });
        });
    })
    .catch(console.error);