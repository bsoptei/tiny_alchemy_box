import('./tiny_alchemy_box')
    .then(wasm => {
        const renderButton = document.getElementById('render');
        const scriptBox = document.getElementById('tab_script');
        const positionIndicator = document.getElementById('cursor_position');

        renderButton.addEventListener('click', () => {
            const inputText = scriptBox.value;
            if (inputText.length > 0) {
                wasm.process(scriptBox.value);
            }
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