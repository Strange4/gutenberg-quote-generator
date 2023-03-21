using MongoDB.Bson;
using System;
using System.Text.RegularExpressions;

namespace QuoteUploader{
    public class Quote{
        
        // Field Properties
        private string Body {get;}
        private int Difficulty {get; set;}
        private int NumberWords {get;}
        private int AverageWordLength {get;}
        private int NumberCharacters {get;}
        private int NumberSpecialCharacters {get;}

        // Special Character list.
        private readonly string  _specialCharacters = @"[,.!?'\""-:]";

        // Constructor that takes in a Quote.
        public Quote(string body){
            Body = SetBody(body);
            int[] words = GetWordsInfo();
            NumberWords = words[0];
            AverageWordLength = words[1];
            int[] nbCharacters = GetCharacters();
            NumberCharacters = nbCharacters[0];
            NumberSpecialCharacters = nbCharacters[1];
            Difficulty = SetDifficulty();
        }

        // Remove all empty lines and tabs or bigger tabs.
        private string SetBody(string body){
            string quote = body.Replace("\n", " ").Replace("\r", " ");
            if (quote[0] == ' '){
                string tempQuote = "";
                for (int i = 1; i < quote.Length; i++){
                    tempQuote += quote[i];
                }
                quote = tempQuote;
            }
            const string multiSpace= @"[ ]{2,}";
            return Regex.Replace(quote.Replace("\t", ""), multiSpace, " ");
        }

        // Returns the number of words in the Quote.
        private int[] GetWordsInfo(){
            var words = Body.Split(" ");
            int average = 0;

            for (int i = 0; i < words.Length; i++){
                average += words[i].Length; 
            }

            return new int[]{words.Length, (average/words.Length)};
        }

        // Returns an array that contains the number of char and special char.
        private int[] GetCharacters(){
            int[] characters = new int[2];
            characters[0] = Body.Length;
            try{
                //characters[1] = Regex.Matches(Body, _specialCharacters).Count;
                var specials = Regex.Matches(Body, _specialCharacters);
                characters[1] = specials.Count;
            } catch (Exception e){
                Console.WriteLine(e.StackTrace);
            }
            return characters;
        }

        // Set the difficulties of the quote depending on words, char, and special char.
        private int SetDifficulty(){
            int difficulty = 1;

            if (NumberWords >= 20)
                difficulty++;
            if (NumberSpecialCharacters >= 2)
                difficulty++;
            if (AverageWordLength >= 6)
                difficulty++;
            if (NumberCharacters >= 80)
                difficulty++;

            return difficulty;
        }

        // Transform the Quote Object to match the database format and returns the Bson of it.
        public BsonDocument QuoteToBson(){
            var data = new BsonDocument { {"quote", Body}, {"difficulty", Difficulty}, {"number_characters", NumberCharacters}, 
            {"number_special_characters", NumberSpecialCharacters}, {"number_words", NumberWords}, 
            {"average_word_length", AverageWordLength} };

            return data;
        }

        // Get the string that displays all the fields of the Quote.
        public override string ToString(){
            return $"Body: {Body}, Diff: {Difficulty}, Words: {NumberWords}, Average: {AverageWordLength}, Char: {NumberCharacters}, Special: {NumberSpecialCharacters}";
        }
    }
}