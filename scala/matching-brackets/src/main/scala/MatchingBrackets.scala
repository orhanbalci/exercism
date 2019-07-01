object MatchingBrackets {
  def isPaired(inp: String): Boolean = {
    var bracketList = List[Char]();
    val brackets = inp
      .foreach(
        s =>
          s match {
            case '{' | '(' | '[' => bracketList = s :: bracketList;
            case '}' =>
              if (bracketList.headOption == Some('{')) {
                bracketList = bracketList.drop(1)
              } else {
                bracketList = s :: bracketList
              }
            case ')' =>
              if (bracketList.headOption == Some('(')) {
                bracketList = bracketList.drop(1)
              } else {
                bracketList = s :: bracketList
              }
            case ']' =>
              if (bracketList.headOption == Some('[')) {
                bracketList = bracketList.drop(1)
              } else {
                bracketList = s :: bracketList
              }
            case _ => ()
          }
      )
    return bracketList.length == 0;
  }

}
