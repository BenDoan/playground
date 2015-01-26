package words

import "testing"

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

//func TestCountWords(t *testing.T) {
	//in, out1, out2 := "Hello this is a simple sentence. Hello.", 7, []string{"is", "a", "simple", "sentence", "hello"}
	//if x, x2 := CountWords(in); x != out1 && x2 != out2 {
		//t.Errorf("CountWords(%v) = %v, want %v", in, x, out1)
	//}

	//in, out1, _= "Hello this is a simple sentence.  Hello.", 6, []string{"Hello", "this", "is", "a", "simple"}
	//if x, _ := CountWords(in); x != 7 {
		//t.Errorf("CountWords(%v) = %v, want %v", in, x, out1)
	//}
//}
