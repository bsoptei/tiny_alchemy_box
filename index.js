import('./tiny_alchemy_box')
    .then(wasm => {
        const renderButton = document.getElementById('render');
        const scriptBox = document.getElementById('tab_script');
        const positionIndicator = document.getElementById('cursor_position');

        renderButton.addEventListener('click', () => {
            const inputText = scriptBox.value;
            if (inputText.length > 0) {
                wasm.process(inputText);
            }
        });

        const boxEffects = ['keydown', 'keyup', 'keypress', 'click', 'mouseover', 'mousedown'];

        boxEffects.forEach((effect) => {
            scriptBox.addEventListener(effect, () => {
                const startPosition = scriptBox.selectionStart;
                const textUntil = scriptBox.value.slice(0, startPosition);
                const barNumber = Math.round((textUntil.match(/[|]/g) || []).length / 2);
                positionIndicator.innerHTML = `Cursor position: ${startPosition} Bar number: ${barNumber}`
            });
        });
    })
    .catch(console.error);