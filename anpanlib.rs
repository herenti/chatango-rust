use std::cmp;

fn g_server(mut group: String) -> i64{

    let weights = [[5, 75],[6, 75],[7, 75],[8, 75],[16, 75],[17, 75],[18, 75],[9, 95],[11, 95],[12, 95],[13, 95],[14, 95],[15, 95],[19, 110],[23, 110],[24, 110],[25, 110],[26, 110],[28, 104],[29, 104],[30, 104],[31, 104],[32, 104],[33, 104],[35, 101],[36, 101],[37, 101],[38, 101],[39, 101],[40, 101],[41, 101],[42, 101],[43, 101],[44, 101],[45, 101],[46, 101],[47, 101],[48, 101],[49, 101],[50, 101],[52, 110],[53, 110],[55, 110],[57, 110],[58, 110],[59, 110],[60, 110],[61, 110],[62, 110],[63, 110],[64, 110],[65, 110],[66, 110],[68, 95],[71, 116],[72, 116],[73, 116],[74, 116],[75, 116],[76, 116],[77, 116],[78, 116],[79, 116],[80, 116],[81, 116],[82, 116],[83, 116],[84, 116]];

    group = group.replace("-","q");
    group = group.replace("_","q");

    let mut a = if group.len() > 6 {
        let mut a = std::cmp::min(3, group.len() - 5);
        let substr = &group[6..6+a];
        let mut a = i64::from_str_radix(substr, 36);
        let mut a = a.unwrap();
        let mut a = a as f64;
        if a <= 1000.0 {
            1000.0
        }
        else {
        a
        }

    }
    else{
        1000.0
    };

    let mut b = std::cmp::min(5, group.len());
    let substr = &group[..b];
    let mut b = i64::from_str_radix(substr, 36).unwrap();
    let mut b = b as f64;
    let num = (b / a) % 1.0;
    let mut anpan = 0.0;
    let mut s_number = 0;
    let total_weight: f64 = weights.iter().map(|a| a[1] as f64).sum();
    for x in weights {
        anpan += x[1] as f64 / total_weight;
        if num <= anpan {
            s_number += x[0];
            break;
        }
    }

    s_number
}




fn main() {

    let to_server: String = String::from("princess-garden");
    let server = format!("s{}.chatango.com", g_server(to_server));
    println!("{}", server);


}

