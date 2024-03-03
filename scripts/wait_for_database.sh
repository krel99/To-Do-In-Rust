#!/bin/bash

# cd .. THIS IS ONLY NEEDED IF NOT RUNNING FROM ROOT
docker-compose up -d

until pg_isready -h localhost -p 5432 -U username
do
  echo "Waiting for postgres"
  sleep 2;
done

echo "docker is now running"
# docker-compose down

# #!/bin/bash

# # cd .. THIS IS ONLY NEEDED IF NOT RUNNING FROM ROOT
# docker-compose up -d

# # Wait for PostgreSQL to be ready
# echo "Waiting for postgres to be ready..."
# until docker-compose exec -T postgres pg_isready -h localhost -p 5435 -U username > /dev/null 2>&1
# do
#   echo "Postgres is not ready yet. Retrying in 2 seconds..."
#   sleep 2
# done

# echo "Postgres is now ready."
# echo "Docker is now running."
