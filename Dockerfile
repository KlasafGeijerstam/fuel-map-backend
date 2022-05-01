FROM klasa/fuel-map-backend-base:latest as builder
RUN mkdir /app
COPY . /app
WORKDIR /app
RUN cargo build --release

FROM klasa/fuel-map-backend-base:latest
RUN mkdir /app
WORKDIR /app
COPY --from=builder /app/target/release/fuel_map_backend .
COPY migrations/ /app/migrations/
ENV DATABASE_URL=""
ENV PORT=80
ENTRYPOINT ["./fuel_map_backend"]
