# toggl

![cURL](cURL.png)

Copy as cURL one of your toggl.com http requests.

Paste this into a text field and extract your cookie to a file like:

```
export TOGGL_COOKIE='_gcl_au=ABC123...'
```

Replace (inside the single quotes) the `_gcl_au=ABC123...` stuff with your cookie value.

# building

```
cargo build
cp targets/debug/toggle-cli .
```

# running

```
./toggl-cli
--task=prefix or (--desc='desc of what I did' --time='when')

./toggl-cli --task='foo'
(this will search your projects and tasks for the string foo)

export TOGGLE_TASK_ID=123
(you get 123 from the step before)

./toggl-cli --desc='i did a lot for project foo.' --time='2024-07-19 16:00'
```
