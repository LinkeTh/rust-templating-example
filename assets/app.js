htmx.defineExtension('bs-validation', {
    onEvent: function (name, evt, data) {
        let form = evt.detail.elt;

        if (!form.hasAttribute('hx-trigger')) {
            form.setAttribute('hx-trigger', 'bs-send');
            form.addEventListener('submit', function (event) {
                if (form.checkValidity()) {
                    htmx.trigger(form, "bs-send");
                }

                let invalidField = form.querySelector(':invalid');
                if (invalidField) {
                    invalidField.focus();
                }

                event.preventDefault();
                event.stopPropagation();

                form.classList.add('was-validated');
            }, false)
        }
    }
});
