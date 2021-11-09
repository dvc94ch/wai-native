use wai_bindgen_rust::Handle;

wai_bindgen_rust::export!("api.witx");

pub struct Api {}

impl api::Api for Api {}

pub struct Greeter {
    lang: api::Lang,
    datetime: Option<api::Datetime>,
    friendly: bool,
    sad: bool,
    agitated: bool,
}

impl Default for Greeter {
    fn default() -> Self {
        Self {
            lang: api::Lang::En,
            datetime: None,
            friendly: false,
            sad: false,
            agitated: false,
        }
    }
}

#[wai_bindgen_rust::async_trait(?Send)]
impl api::Greeter for Greeter {
    fn new() -> Handle<Self> {
        Handle::new(Self::default())
    }

    fn with_config(config: Vec<api::Config>) -> Option<Handle<Self>> {
        let mut greeter = Self::default();
        for config in config {
            match config {
                api::Config::Lang(lang) => greeter.lang = lang,
                api::Config::Datetime(datetime) => greeter.datetime = Some(datetime),
                api::Config::Mode(mode) => {
                    greeter.friendly = mode & api::MODE_FRIENDLY > 0;
                    greeter.sad = mode & api::MODE_SAD > 0;
                    greeter.agitated = mode & api::MODE_AGITATED > 0;
                }
            }
        }
        Some(Handle::new(greeter))
    }

    fn greet(&self) -> Result<String, String> {
        let mut greeting = match self.lang {
            api::Lang::En => "hello world".to_string(),
            api::Lang::De => "hallo welt".to_string(),
        };
        if let Some(datetime) = self.datetime {
            let (day, month, year) = datetime.date;
            let (hour, min, sec) = datetime.time;
            greeting.push_str(&format!(
                " {}.{}.{} {}:{}:{}",
                day, month, year, hour, min, sec
            ));
        }
        if self.agitated {
            greeting.push('!');
        }
        if self.friendly {
            greeting.push_str(" :)");
        }
        if self.sad {
            greeting.push_str(" :(");
        }
        Ok(greeting)
    }

    /*async fn async_greet(&self) -> String {
        self.greet().unwrap()
    }*/
}
