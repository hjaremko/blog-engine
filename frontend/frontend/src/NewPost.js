import React, {Component} from "react";
import axios from 'axios';
import Cookies from 'universal-cookie';

const cookies = new Cookies();

class NewPost extends Component {
    constructor(props) {
        super(props);

        this.state = {
            titleInput: "",
            contentInput: ""
        };

        this.updateTitleInput = this.updateTitleInput.bind(this);
        this.updateContentInput = this.updateContentInput.bind(this);
        this.sendNewPost = this.sendNewPost.bind(this);
    }

    sendNewPost() {
        const headers = {
            'Content-Type': 'application/json',
            'Authorization': 'Bearer ' + cookies.get('token')
        };

        // let url = '/api/posts/';
        let url = 'http://localhost:8000/api/posts/';
        axios.post(url, {
            "title": this.state.titleInput,
            "author_id": 1,
            "content": this.state.contentInput
        }, {headers}).then(res => {
                this.props.history.push('/');
            }
        ).catch(function (error) {
            console.log('Error: ', error.message);
        })
    }

    updateTitleInput(e) {
        this.setState({titleInput: e.target.value});
    }

    updateContentInput(e) {
        this.setState({contentInput: e.target.value});
    }

    render() {
        return (
            <div>
                <div className="mb-3">
                    <label form="exampleFormControlInput1" className="form-label">Tytuł</label>
                    <input onChange={this.updateTitleInput} type="text" className="form-control"
                           id="exampleFormControlInput1"
                           placeholder="Tytuł"/>
                </div>
                <div className="mb-3">
                    <label form="exampleFormControlTextarea1" className="form-label">Treść</label>
                    <textarea onChange={this.updateContentInput} className="form-control"
                              id="exampleFormControlTextarea1" rows="12"/>
                </div>

                <button onClick={this.sendNewPost} type="button" className="btn btn-primary btn-lg">Wyślij</button>
            </div>
        );
    }
}

export default NewPost;