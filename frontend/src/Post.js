import React, {Component} from "react";
import axios from 'axios';
import {Parallax} from 'react-parallax';
import Cookies from 'universal-cookie';

const cookies = new Cookies();

class Post extends Component {
    constructor(props) {
        super(props);

        this.state = {
            post: null,
            comments: [],
            input: ""
        };

        this.renderBlogPost = this.renderBlogPost.bind(this);
        this.updateInput = this.updateInput.bind(this);
    }

    updateInput(e) {
        this.setState({input: e.target.value})
    }

    getComments(postId) {
        // let url = `http://localhost:8000/api/posts/comments/${postId}`;
        let url = `/api/posts/comments/${postId}`;

        axios.get(url)
            .then(res => {
                let comments = res.data;
                let newComment = {[postId]: comments};
                let newComments = {...this.state.comments, ...newComment};
                this.setState({comments: newComments});
            })
            .catch((e) => {
                console.log('Error: ', e.message);
            })
    }

    renderParallax(content) {
        return (
            <div>
                <Parallax blur={0} bgImage="/img/galaxy.jfif" bgImageAlt="galaxy" strength={600}
                          className="parallax-image">
                    <div className="par-header">
                        {content}
                    </div>
                </Parallax>
            </div>
        );
    }

    renderComment(comment, id) {
        return (
            <div key={id}>
                <div className='titlebar'>
                    <span className='post-author'><i className="fas fa-user-alt"></i> {comment.author.name}</span>
                    <span className='post-date'>{comment.date} <i className="far fa-calendar-alt"></i></span>
                </div>
                {comment.content}
            </div>
        )
    }

    renderComments(idx) {
        let comment = this.state.comments[idx];

        if (comment == null) {
            return;
        }

        return comment.map((comment) => {
            return (
                <div key={comment.id}>
                    <div>
                        {this.renderComment(comment, comment.id)}
                    </div>
                </div>
            )
        });
    }

    submitComment(idx) {
        const headers = {
            'Content-Type': 'text/plain',
            'Authorization': 'Bearer ' + cookies.get('token')
        };

        // let url = `http://localhost:8000/api/posts/comments/${idx}`;
        let url = `/api/posts/comments/${idx}`;
        axios.post(url,
            this.state.input
            , {headers}).then(res => {
                this.getComments(idx);
                this.setState({input: ""});
            }
        ).catch(function (error) {
            console.log('Error: ', error.message);
        })
    }

    renderSubmitForm(idx) {
        let token = cookies.get('token');
        if (token == null) {
            return;
        }

        return (
            <div>
                <div className="input-group mb-3">
                    <input type="text" className="form-control" placeholder="Nowy komentarz"
                           aria-label="New comment" aria-describedby="button-addon2"
                           onChange={this.updateInput}
                    >
                    </input>
                    <button className="btn btn-outline-secondary" type="button" id="button-addon2"
                            onClick={() => this.submitComment(idx)}
                            onSubmit={() => this.submitComment(idx)}
                    >Dodaj
                    </button>
                </div>
            </div>
        )
    }

    showComments(idx) {

        return (
            <div>
                {this.renderSubmitForm(idx)}
                {this.renderComments(idx)}
            </div>
        )
    }

    renderPost(post, key) {
        function addHtmlNewlines(content) {
            let newContent = "";
            let objects = content.split("\n");

            for (let i = 0; i < objects.length; i++) {
                newContent += objects[i];
                newContent += "<br>";
            }

            return newContent;
        }

        return (
            <div className="blogpost" key={key}>
                <div className='post-header'>
                    {this.renderParallax(post.title)}
                    <div className='titlebar'>
                        <span className='post-author'><i className="fas fa-user-alt"></i> {post.author.name}</span>
                        <span className='post-date'>{post.date} <i className="far fa-calendar-alt"></i></span>
                    </div>
                </div>
                <div className='post-content' dangerouslySetInnerHTML={{__html: addHtmlNewlines(post.content)}}/>
                {this.showComments(key)}
            </div>
        )
    }

    renderBlogPost() {
        let post = this.props.location.postContent;

        return (
            <div className="posts">
                {this.renderPost(post, post.id)}
            </div>
        )
    }

    componentDidMount() {
        let post = this.props.location.postContent;
        this.getComments(post.id);
    }

    render() {
        return (
            <div>
                {this.renderBlogPost()}
            </div>
        );
    }
}

export default Post;