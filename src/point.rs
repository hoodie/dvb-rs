use serde::{
    Deserialize, Serialize,
    de::{self, Deserializer, Visitor},
    ser::Serializer,
};
use serde_json::Value;

use std::{error::Error, fmt, result, str::FromStr, string::ToString};

use crate::{
    DvbResponse,
    error::Result,
    poi::{PoiId, PoiType},
};

#[derive(Debug)]
pub struct Point {
    pub id: String,
    pub city: String,
    pub name: String,
    pub coords: (i64, i64),
    pub r#type: PoiType,
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();
        let point = Point {
            id: parts[0].into(),
            city: parts[2].into(),
            name: parts[3].into(),
            coords: (parts[4].parse()?, parts[5].parse()?),
            r#type: PoiId::from_str(parts[0]).unwrap().r#type,
        };

        Ok(point)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Point {
            id,
            city,
            name,
            coords: (lon, lat),
            ..
        } = self;
        write!(f, "{id}||{city}|{name}|{lon}|{lat}|0||")
    }
}

impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PointVisitor)
    }
}

struct PointVisitor;
impl<'de> Visitor<'de> for PointVisitor {
    type Value = Point;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a string that follows that \"id||city|name||lon|lat|0||\" format "
        )
    }

    fn visit_str<E>(self, s: &str) -> result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Point::from_str(s) {
            Ok(p) => Ok(p),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config<'a> {
    pub query: &'a str,
    pub limit: Option<u32>,
    pub stops_only: bool,
    pub assigedstops: bool,
    pub dvb: bool,
    pub format: Format,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Self {
            query: Default::default(),
            limit: Default::default(),
            stops_only: Default::default(),
            assigedstops: Default::default(),
            dvb: true,
            format: Format::Json,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub enum Format {
    #[default]
    Json,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Found {
    pub point_status: String,
    pub points: Vec<Point>,
}

const POINT_FINDER_URL: &str = "https://webapi.vvo-online.de/tr/pointfinder";

pub async fn point_finder<'a>(config: &Config<'a>) -> Result<DvbResponse<Found>> {
    let response: Value = reqwest::Client::new()
        .post(POINT_FINDER_URL)
        .json(&config)
        .send()
        .await?
        .json()
        .await?;

    Ok(serde_json::from_value(response)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_city() {
        let point = Point::from_str("33000028|||Hauptbahnhof|5657516|4621644|0||").unwrap();
        assert_eq!(point.city, "");

        let point = Point::from_str("14991850||Wroclaw|Glowny, (Hauptbahnhof)|5674722|4852808|0||")
            .unwrap();
        assert_eq!(point.city, "Wroclaw");
    }

    #[test]
    fn from_string() {
        let points = [
            "33000028|||Hauptbahnhof|5657516|4621644|0||",
            "9004138||Bedburg-Hau|Bahnhof|5752056|4097961|0||",
            "9003735||Hauenstein (Pfalz)|Bahnhof|5459965|4196271|0||",
            "9001411||Hauptstuhl|Bahnhof|5483667|4172880|0||",
            "9001238||Hausach|Bahnhof|5356718|4216759|0||",
            "9002728||Hausen (b Düren)|Bahnhof|5627547|4110803|0||",
            "9022501||Hausen im Wiesental|Bahnhof|0|0|0||",
            "9003343||Hausham (Obb)|Bahnhof|5290276|4488112|0||",
            "9003359||Seeshaupt|Bahnhof|5298524|4446719|0||",
            "9020868||Hausen (Geisingen)|Bahnhof Geisingen-Hausen|0|0|0||",
            "9003540||Kassel|Bahnhof Harleshausen|5691680|4322551|0||",
            "9003870||Harthaus|Bahnhof Harthaus|5333001|4454520|0||",
            "9000593||Augsburg|Bahnhof Haunstetterstraße|5358103|4418613|0||",
            "9003275||Haupeltshofen|Bahnhof Haupeltshofen|5339307|4379827|0||",
            "9021658||Garmisch-Partenkirchen|Bahnhof Hausberg|5260944|4431670|0||",
            "9003270||Hausen (bei Mindelheim)|Bahnhof Hausen|5330292|4387136|0||",
            "9003574||Hausen-Arnsbach|Bahnhof Hausen|5580473|4251320|0||",
            "9004806||Hausen im Tal|Bahnhof Hausen i Tal|5331160|4278592|0||",
            "9002942||Hausen-Raitbach|Bahnhof Hausen-Raitbach|5290694|4188233|0||",
            "9021772||Hauenstein (Pfalz)|Bahnhof Mitte|0|0|0||",
            "9003606||Hainhausen|Bahnhof Rodgau-Hainhausen|5549556|4276301|0||",
            "9002598||Hagen|Bahnhof Wehringhausen|5700819|4183404|0||",
            "9029086||Asbach (Bäumenheim)|Bahnhof/Hauptstraße|5394402|4412508|0||",
            "14930000||Ceská Lípa|Ceska Lipa hl.n. (Hauptbahnhof)|5618592|4679185|0||",
            "14991850||Wroclaw|Glowny, (Hauptbahnhof)|5674722|4852808|0||",
            "9000534||Aachen|Hauptbahnhof|5642476|4083477|0||",
            "9000881||Aschaffenburg|Hauptbahnhof|5542152|4295253|0||",
            "9000589||Augsburg|Hauptbahnhof|5359223|4417532|0||",
            "9001476||Bad Friedrichshall|Hauptbahnhof|5458765|4296147|0||",
            "9004734||Baranovitschi|Hauptbahnhof|0|0|0||",
            "9000906||Bayreuth|Hauptbahnhof|5534910|4469959|0||",
            "9000615||Berchtesgaden|Hauptbahnhof|5276960|4575194|0||",
            "9002022||Bielefeld|Hauptbahnhof|5771833|4262169|0||",
            "9000212||Bingen am Rhein|Hauptbahnhof|5545050|4204877|0||",
            "9000159||Bochum|Hauptbahnhof|5715719|4168395|0||",
            "9000536||Bonn|Hauptbahnhof|5633304|4154040|0||",
            "9000540||Boppard|Hauptbahnhof|5575462|4185232|0||",
            "9002477||Bottrop|Hauptbahnhof|5720648|4148823|0||",
            "40006280||Brandenburg|Hauptbahnhof|0|0|0||",
            "9004387||Bratislava|Hauptbahnhof|5348142|4880157|0||",
            "9000454||Braunschweig|Hauptbahnhof|5791971|4400380|0||",
            "9000445||Bremen|Hauptbahnhof|5888268|4286579|0||",
            "9000448||Bremerhaven|Hauptbahnhof|5939089|4274676|0||",
            "9004385||Brno|Hauptbahnhof|5460632|4836289|0||",
            "9010228||Bydgoszcz|Hauptbahnhof|0|0|0||",
            "9000160||Castrop-Rauxel|Hauptbahnhof|5725910|4174630|0||",
            "36030062||Chemnitz|Hauptbahnhof|5634216|4565661|0||",
            "40011134||Cottbus|Hauptbahnhof|5737688|4660669|0||",
            "9000230||Darmstadt|Hauptbahnhof|5531673|4257850|0||",
            "99204383||Decín|Hauptbahnhof|5628720|4655353|0||",
            "9000913||Deggendorf|Hauptbahnhof|5411750|4569815|0||",
            "9001739||Dessau|Hauptbahnhof|5745110|4516297|0||",
            "9000163||Dortmund|Hauptbahnhof|5719042|4184993|0||",
            "9000161||Duisburg|Hauptbahnhof|5712448|4136820|0||",
            "36055998||Döbeln|Hauptbahnhof|5666309|4576805|0||",
            "9000545||Düsseldorf|Hauptbahnhof|5689003|4136516|0||",
            "40008814||Eberswalde|Hauptbahnhof|0|0|0||",
            "9000461||Emden|Hauptbahnhof|5925987|4180372|0||",
            "31205565||Erfurt|Hauptbahnhof|0|0|0||",
            "9002406||Eschweiler (Rheinl)|Hauptbahnhof|5646658|4095139|0||",
            "9000164||Essen|Hauptbahnhof|5713656|4153658|0||",
            "9001221||Frankenthal (Pfalz)|Hauptbahnhof|5495153|4235911|0||",
            "9000256||Frankfurt (Main)|Hauptbahnhof|5557654|4261469|0||",
            "9001212||Freiburg im Breisgau|Hauptbahnhof|5326124|4189781|0||",
            "9001513||Freudenstadt|Hauptbahnhof|5376225|4234702|0||",
            "9000931||Fürth (Bayern)|Hauptbahnhof|5481913|4426892|0||",
            "9004874||Gdynia|Hauptbahnhof|0|0|0||",
            "9000169||Gelsenkirchen|Hauptbahnhof|5719249|4160263|0||",
            "9002586||Gevelsberg|Hauptbahnhof|5698056|4175344|0||",
            "31206460||Gotha|Hauptbahnhof|0|0|0||",
            "9008374||Graz|Hauptbahnhof|5220512|4759587|0||",
            "9000170||Gütersloh|Hauptbahnhof|5758764|4251403|0||",
            "9000174||Hagen|Hauptbahnhof|5701667|4183920|0||",
            "9001773||Halle (Saale)|Hauptbahnhof|5704848|4499173|0||",
            "9000287||Hanau|Hauptbahnhof|5558411|4280513|0||",
            "9000469||Hannover|Hauptbahnhof|5807224|4346277|0||",
            "9001237||Heidelberg|Hauptbahnhof|5479427|4258790|0||",
            "9000029||Heilbronn|Hauptbahnhof|5448877|4296388|0||",
            "9000474||Hildesheim|Hauptbahnhof|5782733|4360074|0||",
            "34000019||Hof (Saale)|Hauptbahnhof|5574595|4494602|0||",
            "9000706||Ingolstadt|Hauptbahnhof|5400927|4458726|0||",
            "9008552||Innsbruck|Hauptbahnhof|5236248|4454775|0||",
            "9001419||Kaiserslautern|Hauptbahnhof|5486249|4193241|0||",
            "9001261||Karlsruhe|Hauptbahnhof|5434712|4236705|0||",
            "9000310||Kassel|Hauptbahnhof|5690043|4325094|0||",
            "9000726||Kempten (Allgäu)|Hauptbahnhof|5287309|4373841|0||",
            "28540044||Kiel|Hauptbahnhof|0|0|0||",
            "9000128||Kiel|Hauptbahnhof|6022138|4378547|0||",
            "9008633||Klagenfurt|Hauptbahnhof|5166673|4677324|0||",
            "9000557||Koblenz|Hauptbahnhof|5588767|4186197|0||",
            "9002452||Krakow|Hauptbahnhof|5577999|5068401|0||",
            "9000559||Krefeld|Hauptbahnhof|5701898|4121747|0||",
            "9000553||Köln|Hauptbahnhof|5657418|4145880|0||",
            "9003743||Landau (Pfalz)|Hauptbahnhof|5458393|4217868|0||",
            "9000731||Landshut|Hauptbahnhof|5378929|4510120|0||",
            "27013000||Leipzig|Hauptbahnhof|5690107|4526669|0||",
            "9008756||Leoben|Hauptbahnhof|5254418|4733381|0||",
            "9000740||Lindau (Bodensee)|Hauptbahnhof|5269889|4325516|0||",
            "9008784||Linz/Donau|Hauptbahnhof|5352793|4670191|0||",
            "9001273||Ludwigshafen (Rhein)|Hauptbahnhof|5488486|4241694|0||",
            "9001284||Lörrach|Hauptbahnhof|5284178|4174269|0||",
            "9000129||Lübeck|Hauptbahnhof|5971574|4412635|0||",
            "9002623||Lünen|Hauptbahnhof|5729713|4190532|0||",
            "9001803||Magdeburg|Hauptbahnhof|5777470|4474554|0||",
            "9000357||Mainz|Hauptbahnhof|5547239|4231911|0||",
            "9001292||Mannheim|Hauptbahnhof|5488491|4244244|0||",
            "9000563||Mönchengladbach|Hauptbahnhof|5688251|4111982|0||",
            "9000179||Mülheim a.d. Ruhr|Hauptbahnhof|5712021|4144658|0||",
            "9000180||Münster (Westf)|Hauptbahnhof|5767106|4200168|0||",
            "9001809||Naumburg (Saale)|Hauptbahnhof|5669804|4485903|0||",
            "9001439||Neunkirchen (Saar)|Hauptbahnhof|5479623|4149745|0||",
            "9000564||Neuss|Hauptbahnhof|5687878|4128738|0||",
            "9001313||Neustadt (Weinstraße)|Hauptbahnhof|5475236|4219754|0||",
            "9001813||Neustrelitz|Hauptbahnhof|5914760|4571676|0||",
            "9000181||Oberhausen (Rheinl)|Hauptbahnhof|5717081|4142541|0||",
            "9000377||Offenbach (Main)|Hauptbahnhof|5556473|4268361|0||",
            "9000500||Osnabrück|Hauptbahnhof|5800498|4231294|0||",
            "9000182||Paderborn|Hauptbahnhof|5735996|4274848|0||",
            "9001055||Passau|Hauptbahnhof|5382787|4607165|0||",
            "9001579||Pforzheim|Hauptbahnhof|5422656|4258373|0||",
            "9003721||Pirmasens|Hauptbahnhof|5461393|4179421|0||",
            "9001702||Poznan|Hauptbahnhof|5818611|4834125|0||",
            "9000184||Recklinghausen|Hauptbahnhof|5731038|4167947|0||",
            "9001083||Regensburg|Hauptbahnhof|5430516|4507397|0||",
            "9002820||Remscheid|Hauptbahnhof|5682326|4164499|0||",
            "9000061||Reutlingen|Hauptbahnhof|5376904|4293856|0||",
            "9001823||Rostock|Hauptbahnhof|5994199|4508722|0||",
            "37110600||Saarbrücken|Hauptbahnhof|0|0|0||",
            "37140000||Saarlouis|Hauptbahnhof|0|0|0||",
            "9009689||Salzburg|Hauptbahnhof|5297738|4578441|0||",
            "9001098||Schweinfurt|Hauptbahnhof|5545883|4372073|0||",
            "9001834||Schwerin|Hauptbahnhof|5944973|4460913|0||",
            "40011239||Schwerin|Hauptbahnhof|0|0|0||",
            "9001357||Sinsheim|Hauptbahnhof|5461719|4272634|0||",
            "9000581||Solingen|Hauptbahnhof|5681487|4150735|0||",
            "9003811||Speyer|Hauptbahnhof|5471379|4240447|0||",
            "9009329||St. Pölten|Hauptbahnhof|5347506|4769489|0||",
            "9002445||Stolberg (Rheinl)|Hauptbahnhof|5644785|4092639|0||",
            "9001843||Stralsund|Hauptbahnhof|6020386|4570199|0||",
            "9000066||Stuttgart|Hauptbahnhof|5409005|4293003|0||",
            "9004722||Szczecin|Hauptbahnhof|5922503|4670407|0||",
            "9004712||Thale|Hauptbahnhof|5734976|4433018|0||",
            "37198600||Trier|Hauptbahnhof|0|0|0||",
            "9001455||Trier|Hauptbahnhof|5527117|4114875|0||",
            "9000094||Tübingen|Hauptbahnhof|5379532|4282608|0||",
            "9001611||Ulm (Donau)|Hauptbahnhof|5364320|4350694|0||",
            "9009514||Villach|Hauptbahnhof|5166070|4641696|0||",
            "9001706||Warszawa|Hauptbahnhof|5826377|5114673|0||",
            "9009585||Wels (OÖ)|Hauptbahnhof|5338417|4650851|0||",
            "9022258||Wien|Hauptbahnhof|5347818|4825454|0||",
            "9000415||Wiesbaden|Hauptbahnhof|5555040|4231226|0||",
            "9000527||Wilhelmshaven|Hauptbahnhof|5938935|4242541|0||",
            "9000194||Witten|Hauptbahnhof|5710454|4175401|0||",
            "9001458||Wittlich|Hauptbahnhof|5549653|4137377|0||",
            "9000530||Wolfsburg|Hauptbahnhof|5811381|4417679|0||",
            "9000430||Worms|Hauptbahnhof|5506157|4236947|0||",
            "9001707||Wroclaw|Hauptbahnhof|5674408|4852839|0||",
            "9000585||Wuppertal|Hauptbahnhof|5691140|4161576|0||",
            "9001152||Würzburg|Hauptbahnhof|5520399|4351504|0||",
            "9003727||Zweibrücken|Hauptbahnhof|5467029|4162110|0||",
            "37170000||Zweibrücken|Hauptbahnhof|0|0|0||",
            "36041032||Zwickau (Sachs)|Hauptbahnhof|5620108|4533825|0||",
            "9001572||Öhringen|Hauptbahnhof|5454787|4318127|0||",
            "9004445||Ústí (nad Labem)|Hauptbahnhof|5615768|4644682|0||",
            "9023012||Würzburg|Hauptbahnhof (Bismarckstraße)|0|0|0||",
            "37130019||Neunkirchen (Saar)|Hauptbahnhof (Brücke), Neunkirchen|0|0|0||",
            "33000036|||Hauptbahnhof (Friedrich-List-Platz)|5657383|4621726|0||",
            "37130000||Neunkirchen (Saar)|Hauptbahnhof (Vorplatz), Neunkirchen|0|0|0||",
            "9021655||Kassel|Hauptbahnhof (tief)|0|0|0||",
            "9000086||Stuttgart|Hauptbahnhof (tief)|5408940|4292890|0||",
            "9021834||Stolberg (Rheinl)|Hauptbahnhof Gleis 27|0|0|0||",
            "33000032|||Hauptbahnhof Nord|5657679|4621791|0||",
            "28510905||Hamburg|Hauptbahnhof Nord|0|0|0||",
            "9029153||München|Hauptbahnhof Nord|5333807|4467402|0||",
            "37110647||Saarbrücken|Hauptbahnhof Nord|0|0|0||",
            "9007240||München|Hauptbahnhof Nord (Arnulfstr.)|5333807|4467289|0||",
            "9029079||Würzburg|Hauptbahnhof Ost|5520225|4351550|0||",
            "9007705||Ludwigshafen (Rhein)|Hauptbahnhof RHB|5488607|4241742|0||",
            "9000577||Mönchengladbach|Hauptbahnhof Rheydt|5684462|4111254|0||",
            "28510906||Hamburg|Hauptbahnhof Süd|0|0|0||",
            "9029154||München|Hauptbahnhof Süd|5333565|4467310|0||",
            "9000190||Herne|Hauptbahnhof Wanne-Eickel|5721834|4164733|0||",
            "9029080||Würzburg|Hauptbahnhof West|5520294|4351449|0||",
            "37150000||Homburg (Saar)|Hauptbahnhof, Homburg|0|0|0||",
            "27010811||Leipzig|Hauptbahnhof, Ostseite|5690040|4526950|0||",
            "27012995||Leipzig|Hauptbahnhof, Westseite|5690096|4526485|0||",
            "27015738||Leipzig|Hauptbahnhof/Goethestr.|5689905|4526686|0||",
            "40004621||Potsdam|Hauptbahnhof/H.-Mann-Allee|0|0|0||",
            "9029009||Aschaffenburg|Hauptbahnhof/ROB|5542107|4295077|0||",
            "31205891||Gera|Hauptbahnhof/Theater|0|0|0||",
            "28510002||Hamburg|Hauptbahnhof/ZOB|0|0|0||",
            "37100007||Saarbrücken|Hauptgüterbahnhof|0|0|0||",
            "37110806||Saarbrücken|Hauptgüterbahnhof|0|0|0||",
            "28541026||Hamburg|Hausbrucher Bahnhofstraße|0|0|0||",
            "14960000||Hradec Králové|Hradec Kralove hl.n. (Hauptbahnhof)|5571224|4772063|0||",
            "9023039||Kassel|KS Hauptbahnhof Nord|0|0|0||",
            "14967500||Mladá Boleslav|Mlada Boleslav hl.n. (Hauptbahnhof)|5589689|4705287|0||",
            "9000689||München|München Hauptbahnhof|5333694|4467285|0||",
            "9000703||München|München Hauptbahnhof tief|5333781|4467339|0||",
            "14920100||Praha|Praha hl.n.(Hauptbahnhof)|5552510|4674464|0||",
            "40004846||Potsdam|S Hauptbahnhof|0|0|0||",
            "40004435||Potsdam|S Hauptbahnhof/Nord ILB|0|0|0||",
            "40003201||Berlin|S+U Hauptbahnhof|0|0|0||",
            "40003200||Berlin|S+U Hauptbahnhof (tief)|0|0|0||",
            "40011109||Cottbus|Stadtring/Hauptbahnhof|5737574|4660972|0||",
            "31206354||Gotha|Südstraße (Hauptbahnhof)|0|0|0||",
            "40011073||Cottbus|Thiemstr./Hauptbahnhof|5737501|4660898|0||",
            "40010963||Cottbus|Vetschauer Straße/Hauptbahnhof|5737609|4660748|0||",
            "40003205||Berlin|Washingtonplatz/Hauptbahnhof|0|0|0||",
            "31206157||Weimar (Thür)|Weimar, Hauptbahnhof|0|0|0||",
            "27011412||Leipzig|Wintergartenstr./ Hauptbahnhof|5689917|4526825|0||",
            "28500001||Lübeck|ZOB/Hauptbahnhof|0|0|0||",
        ];

        points
            .iter()
            .map(|p| (Point::from_str(p), p))
            .for_each(|(p, s)| assert_eq!(&p.unwrap().to_string(), s));
    }
}
