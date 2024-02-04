import json
import requests_cache
import datetime
import croniter
from pyspark.sql import SparkSession
from pyspark.sql.functions import udf, explode, lit, flatten
from pyspark.sql.types import ArrayType, StructType, StructField, StringType, IntegerType

spark = SparkSession.builder.appName("REST_API_with_PySpark_DF").getOrCreate()

def fetch_data(function: str, symbol: str, output_size: str, api_key: str):
    endpoint = "https://www.alphavantage.co/query?function={function}&symbol={symbol}&outputsize={output_size}&apikey={api_key}".format(
        function = function
        , symbol = symbol
        , output_size = output_size
        , api_key = api_key
    )

    now = datetime.datetime.now()
    cron = croniter.croniter('0 9 * * *', now)

    session = requests_cache.CachedSession(
        "{function}_{symbol}_{output_size}_cache"
        .format(
            function = function
            , symbol = symbol
            , output_size = output_size
        )
        , expire_after = cron.get_next(datetime.datetime)
    )

    response = session.get(endpoint)

    with open('response.json', 'w') as f:
        json.dump(response.json(), f)
    
    return True  # assuming API returns a list of records

def flatten_dataframe(df):
    flat_df = df.select(
        "`Time Series (Daily)`.*"
    )

    flat_df = df.select(flatten(flat_df).alias('r')).collect()

    return flat_df

json_string = fetch_data("TIME_SERIES_DAILY", "IBM", "full", "demo")

df = spark.read.json(path="response.json", multiLine=True)
with open('format.txt', 'w') as f:
    f.write(df._jdf.schema().treeString())

flatten_dataframe(df).to_csv("out.csv", index=False)

