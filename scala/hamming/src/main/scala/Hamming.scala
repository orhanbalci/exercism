object Hamming {
 def distance(s : String, t : String ) : Option[Int] = {
    if (s.length() != t.length())
      return None;

    return Some(s.zip(t).count{case (a,b) => a != b });

 }
}
