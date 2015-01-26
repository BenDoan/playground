// Provides a set of tools for dealing with words
package words

// ReverseString returns the reverse of s
func ReverseString(s string) (result string) {
	for _, v := range s {
		result = string(v) + result
	}
	return
}

// IsVowel returns true if r is a vowel, else false
func IsVowel(r rune) (result bool) {
    result = false;
    if r == 97 || r == 101 || r == 105 || r == 111 || r == 117 {
        result = true
    }
    return
}

// NumVowels returns the number of vowels in s
func NumVowels(s string) (result int) {
    result = 0;

	for _, v := range s {
		if IsVowel(v){
			result++
		}
	}
	return
}

// IsPalindrome returns true if s is a palindrome, else false
func IsPalindrome(s string) (result bool) {
	for i := 0; i < len(s)/2; i++ {
		if s[i] != s[len(s)-i-1] {
			return false
		}
	}
	return true
}

