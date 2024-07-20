## **What Is This?**
This is a minimal example of creating a Message Queueing Telemetry Transport (MQTT) Publisher and Subscriber. 

## **Build** :hammer:

1. Build the Rust binaries.

```bash
cargo build --release
```

2. Build an MQTT Broker (Server) to connect both MQTT Publisher and Subscriber.

```bash
cd $HOME
```

```bash
git clone https://github.com/bytebeamio/rumqtt.git --single-branch --branch main --depth 1
```

```bash
cd $HOME/rumqtt && docker build -t rumqttd .
```

## **Run**

Run the following command for subscribing to and publishing MQTT messages synchronously.

```bash
./target/debug/syncpubsub
```

Run the following command for subscribing to and publishing MQTT messages asynchronously using the `tokio` library to manage asynchronous tasks.

```bash
./target/debug/asyncpubsub
```

## **Verify** :white_check_mark:

Run a MQTT Broker using the command below:

```bash
docker run -it --rm \
	--name mqtt_broker_c \
	-p 1883:1883 \
	-p 1884:1884 \
	-v ./rumqttd/rumqttd.toml:/rumqttd.toml \
	rumqttd -c /rumqttd.toml
```

> The default config from the rumqttd repo sets up endpoints for 2 common MQTT versions: MQTT v4.1 on port 1883, and MQTT v5 on port 1884. 

Running `./target/debug/syncpubsub` in a new terminal, you should then see something similar to below:

```bash
0. Notification = Incoming(ConnAck(ConnAck { session_present: false, code: Success }))
1. Notification = Outgoing(Subscribe(1))
2. Notification = Outgoing(Publish(2))
3. Notification = Outgoing(Publish(3))
4. Notification = Outgoing(Publish(4))
5. Notification = Outgoing(Publish(5))
6. Notification = Outgoing(Publish(6))
7. Notification = Outgoing(Publish(7))
8. Notification = Outgoing(Publish(8))
9. Notification = Outgoing(Publish(9))
10. Notification = Outgoing(Publish(10))
11. Notification = Outgoing(Publish(11))
12. Notification = Incoming(SubAck(SubAck { pkid: 1, return_codes: [Success(AtMostOnce)] }))
13. Notification = Incoming(PubAck(PubAck { pkid: 2 }))
14. Notification = Incoming(PubAck(PubAck { pkid: 3 }))
15. Notification = Incoming(PubAck(PubAck { pkid: 4 }))
16. Notification = Incoming(PubAck(PubAck { pkid: 5 }))
17. Notification = Incoming(PubAck(PubAck { pkid: 6 }))
18. Notification = Incoming(PubAck(PubAck { pkid: 7 }))
19. Notification = Incoming(PubAck(PubAck { pkid: 8 }))
20. Notification = Incoming(PubAck(PubAck { pkid: 9 }))
21. Notification = Incoming(PubAck(PubAck { pkid: 10 }))
22. Notification = Incoming(PubAck(PubAck { pkid: 11 }))
23. Notification = Incoming(Publish(Topic = hello/0/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 0))
24. Notification = Incoming(Publish(Topic = hello/1/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 1))
25. Notification = Incoming(Publish(Topic = hello/2/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 2))
26. Notification = Incoming(Publish(Topic = hello/3/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 3))
27. Notification = Incoming(Publish(Topic = hello/4/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 4))
28. Notification = Incoming(Publish(Topic = hello/5/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 5))
29. Notification = Incoming(Publish(Topic = hello/6/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 6))
30. Notification = Incoming(Publish(Topic = hello/7/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 7))
31. Notification = Incoming(Publish(Topic = hello/8/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 8))
32. Notification = Incoming(Publish(Topic = hello/9/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 9))
33. Notification = Outgoing(PingReq)
34. Notification = Incoming(PingResp)
```

Running `./target/debug/asyncpubsub` in a new terminal, you should then see something similar to below:

```bash
Event = Incoming(ConnAck(ConnAck { session_present: false, code: Success }))
Event = Outgoing(Subscribe(1))
Event = Outgoing(Publish(2))
Event = Incoming(SubAck(SubAck { pkid: 1, return_codes: [Success(AtMostOnce)] }))
Event = Outgoing(PubRel(2))
Event = Incoming(PubRec(PubRec { pkid: 2 }))
Event = Incoming(PubComp(PubComp { pkid: 2 }))
Event = Incoming(Publish(Topic = hello/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 1))
Event = Outgoing(Publish(3))
Event = Outgoing(PubRel(3))
...
Event = Outgoing(Publish(11))
Event = Outgoing(PubRel(11))
Event = Incoming(PubRec(PubRec { pkid: 11 }))
Event = Incoming(PubComp(PubComp { pkid: 11 }))
Event = Incoming(Publish(Topic = hello/world, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 10))
```

To kill the MQTT Broker, run:

```bash
docker kill mqtt_broker_c
```

## **References**

- [How to Use MQTT in Rust with Rumqttc client](https://www.emqx.com/en/blog/how-to-use-mqtt-in-rust#choosing-a-rust-based-mqtt-library)
- [Set up Rust MQTT Server](https://www.darrik.dev/writing/setup-rust-mqtt-server/)
- [rumqtt](https://github.com/bytebeamio/rumqtt)
- [rumqttc](https://docs.rs/rumqttc/latest/rumqttc/index.html)
- [rumqttd](https://docs.rs/rumqttd/latest/rumqttd/)