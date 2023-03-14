using System;
using System.IO;

namespace QuoteUploader
{
    class Program
    {
        static void Main(string[] args)
        {
            // Let's user interact with the program.
            CLI();
        }

        private static void CLI(){
            try{
                // Connect to the database.
                Database database = new Database();
                database.Connect();

                Console.WriteLine("Welcome to QuoteUploader.");

                char userInput = '!';
                while(userInput != '0'){
                    Console.WriteLine("\nWhat would you like to do?");
                    Console.WriteLine("Press '1' to parse data from a txt file.");
                    Console.WriteLine("Press '0' to exit the program.");

                    string choice = Console.ReadLine();
                    userInput = choice[0];

                    switch (userInput){
                        case '1':
                            Console.WriteLine("Enter the name of your txt (write the name as it is) to parse data.");
                            Console.WriteLine("Filename needs to match all the lower and uppercase.");
                            Console.WriteLine("Filename does not need the .txt at the end. Simply the name is needed.");
                            string txtName = Console.ReadLine();
                            UploadQuotes(database, txtName);
                            break;
                        case '0':
                            userInput = '0';
                            break;
                        default:
                        Console.WriteLine("Please enter '1' or '0', any other character is invalid.");
                            break;
                    }
                }
            } catch (Exception e){
                Console.WriteLine(e.StackTrace);
            }
            Console.WriteLine("Thank you for using QuoteUploader Program, have a good day!!!");   
        }

        private static void UploadQuotes(Database database, string txtName){
            try{
                // Get the txt file to parse data.
                string sCurrentDirectory = AppDomain.CurrentDomain.BaseDirectory;
                string fileName = @"..\..\..\QuoteText\" + txtName + ".txt";              
                string sFile = System.IO.Path.Combine(sCurrentDirectory, fileName);
                string sFilePath = Path.GetFullPath(sFile);

                // Parse the data into Quote Object.
                QuoteList quotes = new QuoteList(sFilePath);

                // Insert quotes to db.
                for (int i = 0; i < quotes.Quotes.Length; i++){
                    database.Insert(quotes.Quotes[i].QuoteToBson());
                }

                Console.WriteLine($"All quotes have been uploaded to {Secret.DATABASE_COLLECTION}.");

            } catch (Exception e){
                Console.WriteLine(e.StackTrace);
                Console.WriteLine("\n\nCould not find the file, please make sure it is spelled correctly.");
                Console.WriteLine("Make sure to not include .txt at the end of the filename.");
            }
        }
    }
}
