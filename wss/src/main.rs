use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

use playback_io::read_file;

/// A WebSocket echo server
fn main () {
    match TcpListener::bind("127.0.0.1:9001")
    {
        Ok( server ) =>
        {
            for stream in server.incoming() {
                spawn (move || {
                    let mut websocket = accept(stream.unwrap()).unwrap();
                    loop {
                        match websocket.read_message()
                        {
                            Ok(val) =>
                            {
                                // String Recieved from client
                                let action = val.to_text().unwrap();

                                if action == "shutdown"
                                {
                                    websocket.close(None);
                                } else {
                                    let path = format!("../data/{}.wav", action); // path = ../data/unreleased_carti.wav

                                    let msg = read_file(&*path).unwrap();

                                    // ------------- This variable and the match function below streams out the wav header
                                    let wav_head = format!("{:?}", msg.0);
                                    match websocket.write_message(Message::Text(wav_head) )
                                    {
                                        Ok(()) =>
                                        {
                                            println!("Success");
                                        },
                                        Err(E) =>
                                        {
                                            println!("ERR: {}", E);
                                        }
                                    }

                                    // ------------- This variable and the match function below streams out the wav body
                                    
                                    let mut n = 0;

                                    let wav_body = msg.1.clone();
                                    let wav_body_length = wav_body.clone().len();
                                    let length_factor = wav_body_length / 16;
                                    while n < wav_body.clone().len()
                                    {
                                        let index = n + length_factor;
                                        let mut vec: Vec<f32> = Vec::new();
                                        vec = wav_body[n..index].to_vec();
                                        let _wav_body_0 = format!("{:?}", vec);
                                        match websocket.write_message(Message::Text(_wav_body_0) )
                                        {
                                            Ok(()) =>
                                            {
                                                let action = val.to_text().unwrap();

                                                if action == "shutdown"
                                                {
                                                    websocket.close(None);
                                                }
                                            },
                                            Err(E) =>
                                            {
                                                println!("ERR: {0} on index {1}", E, n.clone());
                                            }
                                        }
                                        n = n + index;
                                    }
                                }
                            }
                            Err(E) =>
                            {
                                println!("ERR: {}", E);
                            }
                        }
                    }
                });
            }
        },
        Err(E) =>
        {
            println!("ERR: {}", E);
        }
    }
}


/*let _wav_body_0 = format!("{0:?} {1:?} {2:?} {3:?} {4:?} {5:?} {6:?} {7:?} {8:?} {9:?} {10:?} 
                                        {11:?} {12:?} {13:?} {14:?} {15:?} {16:?} {17:?} {18:?} {19:?} {20:?} 
                                        {21:?} {22:?} {23:?} {24:?} {25:?} {26:?} {27:?} {28:?} {29:?} {30:?}
                                        {31:?} {32:?} {33:?} {34:?} {35:?} {36:?} {37:?} {38:?} {39:?} {40:?}
                                        {41:?} {42:?} {43:?} {44:?} {45:?} {46:?} {47:?} {48:?} {49:?} {50:?}
                                        {51:?} {52:?} {53:?} {54:?} {55:?} {56:?} {57:?} {58:?} {59:?} {60:?}
                                        {61:?} {62:?} {63:?}", wav_body[n], wav_body[n+1], wav_body[n+2], wav_body[n+3], wav_body[n+4],
                                        wav_body[n+5], wav_body[n+6], wav_body[n+7], wav_body[n+8], wav_body[n+9], wav_body[n+10],
                                        wav_body[n+11], wav_body[n+12], wav_body[n+13], wav_body[n+14],
                                        wav_body[n+15], wav_body[n+16], wav_body[n+17], wav_body[n+18], wav_body[n+19], wav_body[n+20],
                                        wav_body[n+21], wav_body[n+22], wav_body[n+23], wav_body[n+24],
                                        wav_body[n+25], wav_body[n+26], wav_body[n+27], wav_body[n+28], wav_body[n+29], wav_body[n+30],
                                        wav_body[n+31], wav_body[n+32], wav_body[n+33], wav_body[n+34],
                                        wav_body[n+35], wav_body[n+36], wav_body[n+37], wav_body[n+38], wav_body[n+39], wav_body[n+40],
                                        wav_body[n+41], wav_body[n+42], wav_body[n+43], wav_body[n+44],
                                        wav_body[n+45], wav_body[n+46], wav_body[n+47], wav_body[n+48], wav_body[n+49], wav_body[n+50],
                                        wav_body[n+51], wav_body[n+52], wav_body[n+53], wav_body[n+54],
                                        wav_body[n+55], wav_body[n+56], wav_body[n+57], wav_body[n+58], wav_body[n+59], wav_body[n+60],
                                        wav_body[n+61], wav_body[n+62], wav_body[n+63] );*/