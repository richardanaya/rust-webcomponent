#![recursion_limit="256"]
#[macro_use]
extern crate stdweb;

// This function allows us to define a new html element hooked up to the static members of a type
fn define_web_component<T:WebComponent + 'static>(_:T) {

    // we need to use the array of attribute names we should be observing
    // and pass them in as a joined string since giving arrays to stdweb
    // isn't possible or expensive
    let observed_attributes = T::get_observable_attributes().join(":");

    js! {
        // use a global variable that allows us to give a context for what element
        // is currently handling an event
        window.currentElement = null;

        // create a generated custom element
        class GeneratedCustomElement extends HTMLElement {
          static get observedAttributes() {return (@{observed_attributes}).split(":"); }

          constructor() {
              super();
              window.currentElement = this;
              (@{T::constructor})();
              window.currentElement = null;
          }

          connectedCallback() {
            window.currentElement = this;
            (@{T::connected})();
            window.currentElement = null;
          }

          disconnectedCallback() {
            window.currentElement = this;
            (@{T::disconnected})();
            window.currentElement = null;
          }

          attributeChangedCallback(attributeName, oldValue, newValue) {
            window.currentElement = this;
            (@{T::attribute_changed})(attributeName,oldValue||"",newValue||"");
            window.currentElement = null;
          }
        }

        // tell the dom to associate it with an html tag name
        customElements.define(@{T::get_element_name()}, GeneratedCustomElement);
    }
}

trait WebComponent {
    fn get_element_name() -> &'static str {""}
    fn get_observable_attributes() -> Vec<&'static str> {vec![]}
    fn constructor(){}
    fn connected(){}
    fn disconnected(){}
    fn attribute_changed(_attribute_name:String,_old_value:String,_new_value:String){}
}

struct HelloWorld;

// this is an example web component that does some stuff on web component lifecycles
// since there are defaul definitions in the trait, these aren't really all necessary
impl WebComponent for HelloWorld {
    fn get_element_name() -> &'static str {"hello-world"}

    fn get_observable_attributes() -> Vec<&'static str> {vec!["greeting","name"]}

    fn constructor(){
        js! {
            window.currentElement.innerHTML = @{r#"
                <style>
                    hello-world button {
                        border: solid 1px black;
                        border-radius: 5px;
                        padding: 5px;
                        font-family: arial;
                    }
                </style>
                <button>Hello World!</button>
              "#};
            window.currentElement.addEventListener("click", ()=> alert("🎉🎉🎉"))
        }
    }

    fn connected(){
        js! {
            console.log("connected");
        }
    }

    fn disconnected(){
        js! {
            console.log("disconnected");
        }
    }

    fn attribute_changed(attribute_name:String,old_value:String,new_value:String){
        js! {
            var attr = @{attribute_name};
            var oldVal = @{old_value};
            var newVal = @{new_value};
            if(attr === "greeting"){
                window.currentElement.querySelector("button").innerHTML = newVal + " " + (window.currentElement.getAttribute("name")||"world") + "!";
            }
            if(attr === "name"){
                window.currentElement.querySelector("button").innerHTML = (window.currentElement.getAttribute("greeting")||"Hello") + " "+ newVal + "!";
            }
        }
    }
}

fn main() {
    // get std wb started
    stdweb::initialize();

    // define the web components we will use
    define_web_component(HelloWorld);

    // inject some dom into the body to show it off!
    js! {
        document.body.innerHTML = @{r#"
            <hello-world></hello-world>
            <hello-world greeting="Hola" name="Mundo"></hello-world>
        "#};
    }

    // keep std event going
    stdweb::event_loop();
}
