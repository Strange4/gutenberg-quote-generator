using System;
using System.IO;

namespace QuoteUploader
{
    class Program
    {
        // Change this to what text we want to parse the data from.
        private static readonly string _quoteText = "quote.txt";

        static void Main(string[] args)
        {
            // Connect to the database.
            Database database = new Database();
            database.Connect();

            // Get the txt file to parse data.
            string sCurrentDirectory = AppDomain.CurrentDomain.BaseDirectory;
            string fileName = @"..\..\..\..\" + _quoteText;              
            string sFile = System.IO.Path.Combine(sCurrentDirectory, fileName);
            string sFilePath = Path.GetFullPath(sFile);  

            // Parse the data into Quote Object.
            QuoteList quotes = new QuoteList(sFilePath);

            // Insert quotes to db.
            for (int i = 0; i < quotes.Quotes.Length; i++){
                database.Insert(quotes.Quotes[i].QuoteToBson());
            }
        }
    }
}
