window.onload = updateSlider;
function updateSlider() {
    const slider = document.getElementById("duration");
    const sliderLabel = document.getElementById("sliderLabel");
    if (slider.value == 21) {
        sliderLabel.innerHTML = "Manuell";
    } else {
        sliderLabel.innerHTML = slider.value + " sec";
    }
}

async function deletePreset() {
    const dropdown = document.getElementById("preset");
    const url = 'http://localhost:5000/titles/' + dropdown.value;
    const options = {method: 'DELETE'};

    try {
        let response = await fetch(url, options);
        if (response.ok) {
            location.reload();
        }
        console.error(response.text());
    } catch (error) {
        console.error(error);
    }
}

function buttonBearbeitenClick() {
    const dropdown = document.getElementById("preset");
    const selectedText = dropdown.options[dropdown.selectedIndex].innerHTML;
    const parts = selectedText.split("|");

    document.getElementById("inputName").value = parts[0];
    document.getElementById("inputTyp").value = parts[1];

    const form = document.getElementById("create_edit");
    form.action = "/titles/" + dropdown.value;

    const section = document.getElementById("edit_section");
    section.hidden = false;
}