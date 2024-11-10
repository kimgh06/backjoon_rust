use std::cmp::min;
use std::collections::HashMap;

fn main() {
    // 입력 받기
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let sentence = input.trim().to_string(); // 복사본 생성

    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut words = Vec::new();
    for _ in 0..n {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        words.push(input.trim().to_string());
    }

    // 단어 사전 생성
    let mut word_map = HashMap::new();
    for word in words {
        let sorted_word: String = {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_unstable();
            chars.into_iter().collect()
        };
        word_map
            .entry(sorted_word)
            .or_insert_with(Vec::new)
            .push(word);
    }

    let sentence_len = sentence.len();
    let mut dp = vec![None; sentence_len + 1];
    dp[0] = Some(0); // 초기 비용 0으로 시작

    for i in 0..sentence_len {
        if dp[i].is_none() {
            continue;
        }
        for j in (i + 1)..=sentence_len {
            let sub_sentence: String = sentence[i..j].to_string();
            let mut sorted_sub_sentence: Vec<char> = sub_sentence.chars().collect();
            sorted_sub_sentence.sort_unstable();
            let sorted_sub_sentence: String = sorted_sub_sentence.into_iter().collect();

            if let Some(word_list) = word_map.get(&sorted_sub_sentence) {
                for word in word_list {
                    let cost = calculate_cost(&sub_sentence, word);
                    if let Some(current_cost) = dp[j] {
                        dp[j] = Some(min(current_cost, dp[i].unwrap() + cost));
                    } else {
                        dp[j] = Some(dp[i].unwrap() + cost);
                    }
                }
            }
        }
    }

    if let Some(answer) = dp[sentence_len] {
        println!("{}", answer);
    } else {
        println!("-1");
    }
}

// 두 문자열의 순서 변경 비용 계산
fn calculate_cost(s1: &str, s2: &str) -> usize {
    let mut cost = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            cost += 1;
        }
    }
    cost
}
