function request(event) {

    event.preventDefault();

    var choice = document.getElementById("choice");
    var choiceValue = choice.options[choice.selectedIndex].value;

    window.frames[0].location = "https://gentle-refuge-2601.herokuapp.com/" + choiceValue;

    $.ajaxPrefilter(function(options) {
        if (options.crossDomain && jQuery.support.cors) {
            var http = (window.location.protocol === "http:" ? "http:" : "https:");
            options.url = http + "//cors-anywhere.herokuapp.com/" + options.url;
        }
    });

    $.get(
        "https://gentle-refuge-2601.herokuapp.com/" + choiceValue,
        function (response) {
            if (!response || response.length < 1) {
                console.log("Error");
                $("#json").html("Error");
            } else {
                var json = JSON.parse(decodeURI(response));
                console.log("\nJSON:", json);
                $("#json").html("<pre>" + JSON.stringify(json, null, 4) + "</pre>");
            }
        }
    );

}

function setButtonClickHandler() {
    document.getElementById("button").addEventListener("click", request);
}

window.addEventListener("DOMContentLoaded", setButtonClickHandler);