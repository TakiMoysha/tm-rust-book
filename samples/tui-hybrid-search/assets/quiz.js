/* Reusable quiz widget.
   Markup contract:
     <section class="quiz" data-answer="a">
       <p class="q">Question text?</p>
       <button data-k="a">Option A</button>
       <button data-k="b">Option B</button>
       <button data-k="c">Option C</button>
       <p class="feedback" data-ok="…why correct…" data-bad="…nudge…"></p>
     </section>
   Multiple .quiz blocks per page are fine. No dependencies. */
(function () {
  "use strict";

  function init(quiz) {
    var answer = quiz.getAttribute("data-answer");
    var buttons = Array.prototype.slice.call(quiz.querySelectorAll("button[data-k]"));
    var feedback = quiz.querySelector(".feedback");

    buttons.forEach(function (btn) {
      btn.addEventListener("click", function () {
        if (quiz.classList.contains("done")) return;
        var picked = btn.getAttribute("data-k");
        if (picked === answer) {
          btn.classList.add("correct");
          feedback.textContent = feedback.getAttribute("data-ok") || "Correct.";
          feedback.className = "feedback ok";
          quiz.classList.add("done");
        } else {
          btn.classList.add("wrong");
          feedback.textContent = feedback.getAttribute("data-bad") || "Not this one — try again.";
          feedback.className = "feedback bad";
          setTimeout(function () { btn.classList.remove("wrong"); }, 900);
        }
      });
    });
  }

  document.addEventListener("DOMContentLoaded", function () {
    Array.prototype.forEach.call(document.querySelectorAll(".quiz"), init);
  });
})();
