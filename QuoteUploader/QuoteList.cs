using System;
using System.IO;
using System.Text.RegularExpressions;

namespace QuoteUploader{
    public class QuoteList{

        // Fields for QuoteList
        public Quote[] Quotes {get; private set;}
        private string _text; 

        // Constructor for QuoteList.
        public QuoteList(string path){
            try{
                _text = File.ReadAllText(path);
                SetQuotes();
            } catch (Exception e){
                Console.WriteLine(e.StackTrace);
            }
        }

        // Take the text and seperates it into Quote object.
        private void SetQuotes(){
            string pattern = @"[.!?]";
            var seperators = Regex.Matches(_text, pattern);
            string[] sentences = Regex.Split(_text, pattern);

            Quotes = new Quote[sentences.Length-1];

            for (int i = 0; i < sentences.Length - 1; i++){
                Quotes[i] = new Quote(sentences[i] + seperators[i]);
            }
        }
    }
}