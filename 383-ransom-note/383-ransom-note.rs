impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        ('a'..='z').into_iter()
            .fold(true, |res, c| 
                if res { 
                    magazine.matches(c).count() >= ransom_note.matches(c).count()
                } else { res })
    }
}