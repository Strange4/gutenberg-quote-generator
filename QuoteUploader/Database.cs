using System;
using MongoDB.Driver;
using MongoDB.Bson;

namespace QuoteUploader{
    public class Database{

        public IMongoCollection<BsonDocument> Collection {get; private set;}
  
        // Delete a quote from the database.
        public void DeleteQuotes(){
            Collection.DeleteMany(Builders<BsonDocument>.Filter.Empty);
            Console.WriteLine($"Succesfully Deleted all Quotes from {Secret.DATABASE_COLLECTION}.");
        }

        // Insert to the database
        public void Insert(BsonDocument data){
            Collection.InsertOne(data);
            Console.WriteLine($"Successfully Inserted Data into {Secret.DATABASE_COLLECTION}.");
        }

        // Connect to the database.
        public void Connect(){
            var settings = MongoClientSettings.FromConnectionString(Secret.ATLAS_URI);
            settings.ServerApi = new ServerApi(ServerApiVersion.V1);
            var client = new MongoClient(settings);
            var database = client.GetDatabase(Secret.DATABASE_NAME);
            Collection = database.GetCollection<BsonDocument>(Secret.DATABASE_COLLECTION);
            Console.WriteLine($"Succesfully Connected to {Secret.DATABASE_NAME}");
        }
    }
}