package words

import (
    "testing"
    "reflect"
);

func TestReverseString(t *testing.T) {
	in, out := "hello", "olleh"
	if x := ReverseString(in); x != out {
		t.Errorf("ReverseString(%v) = %v, want %v", in, x, out)
	}

	in, out = "racecar", "racecar"
	if x := ReverseString(in); x != out {
		t.Errorf("ReverseString(%v) = %v, want %v", in, x, out)
	}
}

func TestNumVowels(t *testing.T) {
	in, out := "hello", 2
	if x := NumVowels(in); x != out {
		t.Errorf("NumVowels(%v) = %v, want %v", in, x, out)
	}

	in, out = "aeiou", 5
	if x := NumVowels(in); x != out {
		t.Errorf("NumVowels(%v) = %v, want %v", in, x, out)
	}
}

func TestIsPalindrome(t *testing.T) {
	in, out := "hello", false
	if x := IsPalindrome(in); x != out {
		t.Errorf("IsPalindrome(%v) = %v, want %v", in, x, out)
	}

	in, out = "racecar", true
	if x := IsPalindrome(in); x != out {
		t.Errorf("IsPalindrome(%v) = %v, want %v", in, x, out)
	}
}

func TestGetWordCount(t *testing.T) {
	in, out := "hello world world the is a sentence we we we we", map[string]int{
        "hello": 1,
        "world": 2,
        "the": 1,
        "is": 1,
        "a": 1,
        "sentence": 1,
        "we": 4,
    }

	if x := GetWordCounts(in); !reflect.DeepEqual(x, out) {
		t.Errorf("IsPalindrome(%v) = %v, want %v", in, x, out)
	}
}
