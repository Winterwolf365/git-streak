var buttons = document.getElementsByTagName('h2');
var sections = document.getElementsByTagName('section');

function toggle() {
    for (let i = 0; i < buttons.length; i++) {
        let button = buttons[i];

        if (button.classList.contains('selected')) {
            button.classList.remove('selected');
        } else {
            button.classList.add('selected');
        }
    }

    for (let i = 0; i < sections.length; i++) {
        let section = sections[i];

        if (section.classList.contains('none')) {
            section.classList.remove('none');
        } else {
            section.classList.add('none');
        }
    }
}
