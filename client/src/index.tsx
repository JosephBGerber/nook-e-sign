import './styles.css'
import * as React from 'react';
import {render} from "react-dom";
import {useCallback, useEffect, useReducer, useState} from 'react';

//========== Model

interface Device {
    id: number;
    charge: number;
    image_hash: string;
}

interface Model {
    authenticated: boolean,
    devices: Device[],
    selected_device: number | null
}

const initialModel = {
    authenticated: false,
    devices: [],
    selected_device: null
}

type Msg
    = { type: "AUTHENTICATED" }
    | { type: "SELECT_DEVICE", id: number }
    | { type: "DELETE_DEVICE", id: number }
    | { type: "DESELECT_DEVICE" }
    | { type: "GET_DEVICES", devices: Device[] }

function reducer(model: Model, msg: Msg): Model {
    switch (msg.type) {
        case "AUTHENTICATED":
            return {
                ...model,
                authenticated: true
            }
        case "SELECT_DEVICE":
            if (model.devices && model.selected_device == null) {
                const selected = model.devices.find((device) => device.id == msg.id)

                return {
                    ...model,
                    selected_device: selected ? msg.id : null
                }
            } else {
                return model
            }
        case "DELETE_DEVICE":
            if (model.devices && model.selected_device == null) {
                const devices = model.devices.filter((device) => device.id == msg.id)

                return {
                    ...model,
                    devices: devices,
                }
            } else {
                return model
            }
        case "DESELECT_DEVICE":
            return {
                ...model,
                selected_device: null
            }
        case "GET_DEVICES":
            return {
                ...model,
                devices: msg.devices
            }
        default:
            throw new Error();
    }
}

//========== Components

const App: React.FC<Model> = (initialModel: Model) => {
    const [model, dispatch] = useReducer(reducer, initialModel)

    useEffect(() => {
        if (model.authenticated) {
            const timer = setTimeout(async () => {
                const result = await fetch("client/device");
                if (result.ok) {
                    const devices: Device[] = await result.json();
                    const sorted = devices.sort((a, b) => a.id - b.id);
                    dispatch({type: "GET_DEVICES", devices: sorted});
                }
            }, 1000)

            return () => clearTimeout(timer)
        }
    })

    let content
    if (!model.authenticated) {
        content = <Login dispatch={dispatch}/>
    } else {
        if (model.selected_device) {
            const selected = model.devices.find((device) => device.id == model.selected_device)

            // @ts-ignore
            content = <ImageView dispatch={dispatch} id={selected.id} image_hash={selected.image_hash}/>
        } else {
            content = <table>
                <tr>
                    <th>Id</th>
                    <th>Charge</th>
                    <th>Actions</th>
                </tr>
                <Devices dispatch={dispatch} devices={model.devices}/>
            </table>
        }
    }

    return <div className="mainContainer">
        <div className="mainColumn">
            {content}
        </div>
    </div>;

}


const Devices: React.FC<{ dispatch: React.Dispatch<Msg>, devices: Device[] }> = ({dispatch, devices}) => {
    return <React.Fragment>
        {devices.map((device: Device, index) => {
            return <Row key={index} dispatch={dispatch} {...device}/>;
        })}
    </React.Fragment>;
}

const Row: React.FC<{ dispatch: React.Dispatch<Msg>, id: number, charge: number }> = ({dispatch, id, charge}) => {
    function onClickEdit(event: React.MouseEvent) {
        dispatch({type: "SELECT_DEVICE", id: id})
    }

    function onClickDelete(event: React.MouseEvent) {
        fetch(`client/device/${id}`, {
            method: "DELETE"
        })
            .then(res => res.json())
            .then(
                () => {
                    dispatch({type: "DELETE_DEVICE", id: id})
                }
            )
    }

    return <tr>
        <td>{id}</td>
        <td>{charge}</td>
        <td>
            <button onClick={onClickEdit}>Edit</button>
            <button onClick={onClickDelete}>Delete</button>
        </td>
    </tr>;
}

const Login: React.FC<{ dispatch: React.Dispatch<Msg> }> = ({dispatch}) => {
    const [name, setName] = useState("");
    const [password, setPassword] = useState("");

    const loginRequest = useCallback(async () => {
        const result = await fetch(`client/login?name=${name}&password=${password}`);
        if (result.ok) {
            if (result.status == 200) {
                dispatch({type: "AUTHENTICATED"})
            }
        } else {
            // handle error
        }
    }, [name, password])

    return <div>
        <input type="text" onChange={(event) => {
            setName(event.target.value);
        }} value={name}/>
        <input type="password" onChange={(event) => {
            setPassword(event.target.value);
        }} value={password}/>
        <button onClick={loginRequest}>Login</button>
    </div>
}


const ImageView: React.FC<{ dispatch: React.Dispatch<Msg>, id: number, image_hash: string }> = ({
                                                                                                    dispatch,
                                                                                                    id,
                                                                                                    image_hash
                                                                                                }) => {
    const [angle, setAngle] = useState(0);

    function onClick(event: React.MouseEvent) {
        dispatch({type: "DESELECT_DEVICE"})
    }

    return <div className="imageView" key={id}>
        <div className="imageView__buttons">
            <button onClick={() => {
                setAngle(angle - 90)
            }}>
                Counter-Clockwise
            </button>
            <button onClick={() => {
                setAngle(angle + 90)
            }}>
                Clockwise
            </button>

            <button onClick={onClick}>Close</button>
        </div>


        <form>
            <input name="image" type="file"/>
            <input formAction={`client/device/${id}/image`} formMethod={"POST"} formEncType={"multipart/form-data"}
                   formTarget={"hiddenFrame"}
                   type="submit" value="Submit"/>
        </form>

        {/* We append the image hash so that our image will update if a new image has been uploaded */}
        <img alt={"Image displayed on device " + id} className="imageView__image"
             src={`client/device/${id}/image?hash=${image_hash}`} style={{rotate: `${angle}deg`}}/>

    </div>
}

//========== Entry point

render(
    <App {...initialModel}  />,
    document.getElementById("root"));
