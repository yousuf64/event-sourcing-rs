cosmos-emulator:
	docker run -p 8082:8081 --memory 3g --cpus=4.0 --name=cosmos-emulator --env AZURE_COSMOS_EMULATOR_PARTITION_COUNT=3 --env AZURE_COSMOS_EMULATOR_ENABLE_DATA_PERSISTENCE=true \
    	--interactive --tty mcr.microsoft.com/cosmosdb/linux/azure-cosmos-emulator