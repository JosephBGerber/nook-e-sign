import ReactDOM from 'react-dom';
import React, {useReducer} from 'react';
import {useEffect, useState} from 'react';

//========== Model

interface Device {
    id: number;
    charge: number;
    image_hash: string;
}

interface Model {
    devices: Device[],
    selected_device: number | null
}

const initialModel = {
    devices: [],
    selected_device: null
}

type Msg
    = { type: "SELECT_DEVICE", id: number }
    | { type: "DESELECT_DEVICE" }
    | { type: "GET_DEVICES", devices: Device[] }

function reducer(model: Model, msg: Msg): Model {
    switch (msg.type) {
        case "SELECT_DEVICE":
            if (model.devices) {
                const selected = model.devices.find((device) => device.id == msg.id)

                return {
                    ...model,
                    selected_device: selected ? msg.id : null
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
        const timer = setTimeout(() => {
            fetch("device")
                .then(res => res.json())
                .then(
                    (result: Device[]) => {
                        const sorted = result.sort((a, b) => a.id - b.id)
                        dispatch({type: "GET_DEVICES", devices: sorted})
                    }
                )
        }, 1000)

        // return () => clearTimeout(timer)
    })

    let imageView
    if (model.selected_device) {
        const selected = model.devices.find((device) => device.id == model.selected_device)

        if (selected) {
            imageView = <ImageView dispatch={dispatch} id={selected.id} image_hash={selected.image_hash}/>
        }
    }

    return <React.Fragment>
        <table>
            <tr>
                <th>Id</th>
                <th>Charge</th>
                <th>Actions</th>

            </tr>
            <Devices dispatch={dispatch} devices={model.devices}/>
        </table>
        {imageView}
    </React.Fragment>;
}


const Devices: React.FC<{ dispatch: React.Dispatch<Msg>, devices: Device[] }> = ({dispatch, devices}) => {
    return <React.Fragment>
        {devices.map((device: Device, index) => {
            return <Row key={index} dispatch={dispatch} {...device}/>;
        })}
    </React.Fragment>;
}

const Row: React.FC<{ dispatch: React.Dispatch<Msg>, id: number, charge: number }> = ({dispatch, id, charge}) => {
    function onClick(event: React.MouseEvent) {
        dispatch({type: "SELECT_DEVICE", id: id})
    }

    return <tr>
        <td>{id}</td>
        <td>{charge}</td>
        <td>
            <button onClick={onClick}>Edit</button>
        </td>
    </tr>;
}

const ImageView: React.FC<{ dispatch: React.Dispatch<Msg>, id: number, image_hash: string }> = ({dispatch, id, image_hash}) => {
    const [angle, setAngle] = useState(0);

    function onClick(event: React.MouseEvent) {
        dispatch({type: "DESELECT_DEVICE"})
    }

    return <div key={id}>
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

        <form>
            <input name="image" type="file"/>
            <input formAction={`device/${id}/image`} formMethod={"POST"} formEncType={"multipart/form-data"}
                   formTarget={"hiddenFrame"}
                   type="submit" value="Submit"/>
        </form>

        {/* We append the image hash so that our image will update if a new image has been uploaded */}
        <img alt={"Image displayed on device " + id} src={`device/${id}/image?hash=${image_hash}`}
             style={{rotate: `${angle}deg`}}/>
    </div>
}

//========== Entry point

ReactDOM.render(
    <App {...initialModel}  />,
    document.getElementById("root"));
