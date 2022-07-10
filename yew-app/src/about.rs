use yew::prelude::*;

#[function_component(About)]
pub fn comp() -> Html {
    html! {
      <div class="about">
          <p>{"
            The goal of this project is a raspberry pi device with a camera, that
            films your garden. If it detects a bird, it takes a picture and identifies
            the species of this bird.
            "}
          </p>
          <p>{"
            The device should be connactable via bluetooth or wifi to view the taken
            pictures.
            "}
          </p>
      </div>
    }
}
