use std::collections::HashMap;

// 存储球队进球和失球数据的结构体
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

// 补充哈希映射类型参数：键为球队名称(String)，值为Team结构体
fn build_scores_table(results: String) -> HashMap<String, Team> {
    // 初始化空的哈希映射
    let mut scores: HashMap<String, Team> = HashMap::new();
    
    for r in results.lines() {
        // 按逗号分割每行数据
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        
        // 更新球队1的数据
        let team1 = scores.entry(team_1_name).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team1.goals_scored += team_1_score;
        team1.goals_conceded += team_2_score;
        
        // 更新球队2的数据
        let team2 = scores.entry(team_2_name).or_insert(Team {
            goals_scored: 0,
            goals_conceded: 0,
        });
        team2.goals_scored += team_2_score;
        team2.goals_conceded += team_1_score;
    }
    
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string() + 
            "England,France,4,2\n" + 
            "France,Italy,3,1\n" + 
            "Poland,Spain,2,0\n" + 
            "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());
        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);  // 4 (对法国) + 1 (对德国) = 5
        assert_eq!(team.goals_conceded, 4); // 2 (对法国) + 2 (对德国) = 4
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);  // 未进球
        assert_eq!(team.goals_conceded, 2); // 被波兰进2球
    }
}
