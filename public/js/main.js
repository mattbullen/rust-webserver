function request(event) {

    event.preventDefault();

    var choice = document.getElementById("choice");
    var choiceValue = choice.options[choice.selectedIndex].value;
    
    document.getElementById("frame").style.height = "60px";
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
                console.log("\nResponse:", response);
                var json = JSON.parse(decodeURI(response));
                console.log("JSON.parse():", json);
                $("#json").html("<pre>" + JSON.stringify(json, null, 4) + "</pre>");
            }
        }
    );

}

function init() {
    document.getElementById("frame").style.height = "33px";
    document.getElementById("button").addEventListener("click", request);
}

window.addEventListener("DOMContentLoaded", init);